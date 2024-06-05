# flake8: noqa: PYI021
"""
An optimised rust version of fizzbuzz.

Provides a fizzbuzz() function which will run on multiple CPU cores if needed.

Usage:
    ```
    >>> from fizzbuzz.fizzbuzzo3 import fizzbuzz
    ```
"""

def fizzbuzz(n: int | list[int] | slice) -> str:
    """
    Returns the correct fizzbuzz answer for any number or list/range of numbers.

    This is an optimised algorithm compiled in rust. Large lists will utilise multiple CPU cores for processing.
    Passing a slice, to represent a range, is fastest.

    Arguments:
        n: the number(s) to fizzbuzz

    Returns:
        A `str` with the correct fizzbuzz answer(s).
        In the case of a list or range of inputs the answers will be separated by `, `

    Examples:
        a single `int`:
        ```
        >>> from fizzbuzz.fizzbuzzo3 import fizzbuzz
        >>> fizzbuzz(1)
        '1'
        >>> fizzbuzz(3)
        'fizz'
        ```
        a `list`:
        ```
        from fizzbuzz.fizzbuzzo3 import fizzbuzz
        >>> fizzbuzz([1,3])
        '1, fizz'
        ```
        a `slice` representing a range:
        ```
        from fizzbuzz.fizzbuzzo3 import fizzbuzz
        >>> fizzbuzz(slice(1,4,2))
        '1, fizz'
        >>> fizzbuzz(slice(1,4))
        '1, 2, fizz'
        >>> fizzbuzz(slice(4,1,-1))
        '4, fizz, 2'
        ```
        Note: Slices are inclusive on the left, exclusive on the right and can contain an optional step.
        ///     Negative steps require start > stop, positive steps require stop > start; or else will return an empty list.
    """
