Here are some tips, tricks, and Python features that solve common questions and help you write succinct, easy-to-read code.

## Quick Logic Functions

All of the above examples define named custom logic functions and then pass them to an operator.
Any callable value can be used as-is, though!

This means you can use the following existing callables to help you make code more concise:

- [Built-in functions](https://docs.python.org/3/library/functions.html)
- [Constructors](https://docs.python.org/3/tutorial/classes.html#class-objects)
- [Methods](https://docs.python.org/3/glossary.html#term-method)

You can also use [lambdas](https://docs.python.org/3/tutorial/controlflow.html#lambda-expressions) to quickly define one-off anonymous functions for simple custom logic.

The following sets of examples are equivalent.

```python
from bytewax.dataflow import Dataflow
from bytewax.execution import run_main
from bytewax.inputs import ManualInputConfig
from bytewax.outputs import StdOutputConfig
from bytewax.window import SystemClockConfig, TumblingWindowConfig
from datetime import timedelta, datetime

# For all examples below
def input_builder(worker_index, worker_count, resume_from):
    resume_from = None # Ignore recovery logic
    for e in data:
        yield resume_from, e
```

### For flat map:

```python
def split_sentence(sentence):
    return sentence.split()

data = ["hello world"]
flow = Dataflow()
flow.input("inp", ManualInputConfig(input_builder))

# Operate on input
flow.flat_map(split_sentence)

flow.capture(StdOutputConfig())
run_main(flow)
```

```{testoutput}
hello
world
````

```python
data = ["hello world"]
flow = Dataflow()
flow.input("inp", ManualInputConfig(input_builder))

# Operate on input
flow.flat_map(lambda s: s.split())

flow.capture(StdOutputConfig())
run_main(flow)
```

```{testoutput}
hello
world
```

```python
data = ["hello world"]
flow = Dataflow()
flow.input("inp", ManualInputConfig(input_builder))

# Operate on input
flow.flat_map(str.split)

flow.capture(StdOutputConfig())
run_main(flow)
```

```{testoutput}
hello
world
```

### For reduce window:

```python
def add_to_list(l, items):
    l.extend(items)
    return l

clock = SystemClockConfig()
window = TumblingWindowConfig(length=timedelta(seconds=5))

data = [("a", ["x"]), ("a", ["y"])]
flow = Dataflow()
flow.input("inp", ManualInputConfig(input_builder))

# Operate on input
flow.reduce_window("reduce", clock, window, add_to_list)
flow.capture(StdOutputConfig())

run_main(flow)
```

```{testoutput}
('a', ['x', 'y'])
```

```python
data = [("a", ["x"]), ("a", ["y"])]
flow = Dataflow()
flow.input("inp", ManualInputConfig(input_builder))

# Operate on input
flow.reduce_window("reduce", clock, window, lambda l1, l2: l1 + l2)
flow.capture(StdOutputConfig())

run_main(flow)
```

```{testoutput}
('a', ['x', 'y'])
```

```python
import operator

data = [("a", ["x"]), ("a", ["y"])]
flow = Dataflow()
flow.input("inp", ManualInputConfig(input_builder))

# Operate on input
flow.reduce_window("reduce", clock, window, operator.add)
flow.capture(StdOutputConfig())

run_main(flow)
```

```{testoutput}
('a', ['x', 'y'])
```

## Subflows

If you find yourself repeating a series of steps in your dataflows or want to give some steps a descriptive name, you can group those steps into a **subflow** function which adds a sequence of steps.
You can then call that subflow function whenever you need that step sequence.
This is just calling a function.

```python
def user_reducer(all_events, new_events):
    return all_events + new_events


def collect_user_events(flow, clock, window):
    # event
    flow.map(lambda e: (e["user_id"], [e["type"]]))
    # (user_id, [event])
    flow.reduce_window("reducer", clock, window, user_reducer)
    # (user_id, events_for_user)
    flow.map(lambda e: {"user_id": e[0], "all_events": e[1]})


clock = SystemClockConfig()
window = TumblingWindowConfig(length=timedelta(seconds=5))

data = [{"user_id": "1", "type": "login"}, {"user_id": "1", "type": "logout"}]
flow = Dataflow()
flow.input("inp", ManualInputConfig(input_builder))

# Operate on input
collect_user_events(flow, clock, window)
flow.capture(StdOutputConfig())

run_main(flow)
```

```{testoutput}
{'user_id': '1', 'all_events': ['login', 'logout']}
```
