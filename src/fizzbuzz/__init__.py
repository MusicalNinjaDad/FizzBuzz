"""
A python implementation of fizzbuzz using closures and a dictionary of rules.

Trying to find a pythonic approach was surprisingly difficult as most of what is considered to be pythonic doesn't
make sense in fizzbuzz. In the end I went for an approach using closures, a dictionary lookup and a generator
expression.

## Examples:

```
>>> fizzbuzzer(1)
'1'
```

```
>>> fizzbuzzer(3)
'fizz'
```

"""
from .fizzbuzzer import fizzbuzzer