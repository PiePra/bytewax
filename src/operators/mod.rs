//! Code implementing Bytewax's operators.
//!
//! The two big things in here are:
//!
//!   1. Shim functions that totally encapsulate PyO3 and Python
//!   calling boilerplate to easily pass into Timely operators.
//!
//!   2. Implementation of stateful operators using
//!   [`crate::recovery::StatefulLogic`] and
//!   [`crate::window::WindowLogic`].

use crate::pyo3_extensions::{TdPyAny, TdPyCallable, TdPyIterator};
use crate::try_unwrap;
use crate::{log_func, unwrap_any};
use log::debug;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;

pub(crate) mod fold_window;
pub(crate) mod reduce;
pub(crate) mod reduce_window;
pub(crate) mod stateful_map;

pub(crate) fn map(mapper: &TdPyCallable, item: TdPyAny) -> TdPyAny {
    debug!("{}, mapper:{:?}, item:{:?}", log_func!(), mapper, item);
    Python::with_gil(|py| unwrap_any!(mapper.call1(py, (item,))).into())
}

pub(crate) fn flat_map(mapper: &TdPyCallable, item: TdPyAny) -> TdPyIterator {
    debug!("{}, mapper:{:?}, item:{:?}", log_func!(), mapper, item);
    Python::with_gil(|py| try_unwrap!(mapper.call1(py, (item,))?.extract(py)))
}

pub(crate) fn filter(predicate: &TdPyCallable, item: &TdPyAny) -> bool {
    debug!(
        "{}, predicate:{:?}, item:{:?}",
        log_func!(),
        predicate,
        item
    );
    Python::with_gil(|py| {
        try_unwrap!({
            let should_emit_pybool: TdPyAny = predicate.call1(py, (item,))?.into();
            should_emit_pybool.extract(py).map_err(|_err| {
                PyTypeError::new_err(format!(
                    "return value of `predicate` in filter \
                operator must be a bool; got `{should_emit_pybool:?}` instead"
                ))
            })
        })
    })
}

pub(crate) fn inspect(inspector: &TdPyCallable, item: &TdPyAny) {
    debug!(
        "{}, inspector:{:?}, item:{:?}",
        log_func!(),
        inspector,
        item
    );
    Python::with_gil(|py| unwrap_any!(inspector.call1(py, (item,))));
}

pub(crate) fn inspect_epoch(inspector: &TdPyCallable, epoch: &u64, item: &TdPyAny) {
    debug!(
        "{}, inspector:{:?}, item:{:?}",
        log_func!(),
        inspector,
        item
    );
    Python::with_gil(|py| unwrap_any!(inspector.call1(py, (*epoch, item))));
}
