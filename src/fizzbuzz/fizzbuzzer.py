"""Implements fizzbuzzer() function."""
from collections import OrderedDict
from typing import Any, Callable


def modulo(n: int) -> Callable[[int], bool]:
    """
    Returns a function to test whether a value is directly divisible by n.
    
    Example:
    ```
    >>> even = modulo(2)
    >>> even(4)
    True
    >>> even(3)
    False
    ```
    """
    def _modulo(x: int) -> bool:
        return x % n == 0

    return _modulo


def _joiner(rules: dict) -> Callable[[Any], Any]:
    def __joiner(val: int) -> str:
        result = "".join(rtn for f, rtn in rules.items() if f(val))
        if result:
            return result
        return str(val)

    return __joiner


rules = OrderedDict(
    {
        modulo(3): "fizz",
        modulo(5): "buzz",
    },
)

fizzbuzzer = _joiner(rules)
