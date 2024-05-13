# ruff: noqa: PYI021
def fizzbuzz(n: int | list[int]) -> str:
    """
    Compute the fizzbuzz answer for `n` using a highly efficient algorithm written in rust.

    The correct fizzbuzz answer is the original number, unless divisible by 3 or 5.
    For numbers divisible by 3, the correct answer is 'fizz'.
    For numbers divisible by 5, the correct answer is 'buzz'.
    For numbers divisible by both 3 & 5, the correct answer is 'fizzbuzz'.

    **Note:** Passing a `list` of values to fizzbuzz is more efficient than making multiple calls.
    Larger lists will be processed in parallel on multiple cpu cores.

    Arguments:
        n: either `int` the  single number to fizzbuzz or `list[int]` a list of numbers to fizzbuzz.

    Returns:
        A string representing the fizzbuzz result. If `n` is an integer, the string contains the fizzbuzz 
        answer for that number. If `n` is a list of integers, the string contains the fizzbuzz answers for each number,
        separated by commas and spaces (`, `).

    Examples:
        Using a single value:
        ```
        >>> from fizzbuzz.fizzbuzzo3 import fizzbuzz
        >>> fizzbuzz(1)
        '1'
        >>> fizzbuzz(5)
        'buzz'
        ```

        Using a list:
        ```
        >>> from fizzbuzz.fizzbuzzo3 import fizzbuzz
        >>> fizzbuzz([1, 5])
        '1, buzz'
        ```
    """
