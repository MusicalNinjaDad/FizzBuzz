#ruff: noqa: PYI021
"""Module level Docstring for fizzbuzzo3."""

def fizzbuzz(n: int | list[int]) -> str:
    """
    Returns the correct fizzbuzz answer for any number or list of numbers.

    This is an optimised algorithm compiled in rust. Large lists will utilise multiple CPU cores for processing.

    Arguments:
        n: the number(s) to fizzbuzz

    Returns:
        A `str` with the correct fizzbuzz answer(s).
        In the case of a list of inputs the answers will be separated by `, ` 

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
    """