"""
A python implementation of fizzbuzz using closures and a dictionary of rules.

Trying to find a pythonic approach was surprisingly difficult as most of what is considered to be pythonic doesn't
make sense in fizzbuzz. In the end I went for an approach using closures, a dictionary lookup and a generator
expression.

Usage:
    ```
    >>> from fizzbuzz.fizzbuzzpy import fizzbuzz 
    ```
"""

from collections import OrderedDict
from typing import Callable


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

def fizzbuzz(val: int) -> str:
    """
    Returns the correct fizzbuzz answer for any number.

    ## Examples:

    ```
    >>> fizzbuzz(1)
    '1'
    ```

    ```
    >>> fizzbuzz(3)
    'fizz'
    ```
    """
    rules = OrderedDict(
        {
            modulo(3): "fizz",
            modulo(5): "buzz",
        },
    )

    if result := "".join(rtn for f, rtn in rules.items() if f(val)):
        return result
    return str(val)
