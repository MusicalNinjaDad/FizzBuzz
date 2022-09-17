from collections import OrderedDict
from typing import Any, Callable


def modulo(n: int) -> Callable[[int], bool]:
    def _modulo(x: int) -> bool:
        return x % n == 0
    return _modulo


def joiner(rules: dict) -> Callable[[Any], Any]:
    def _joiner(val):
        result = ''.join(rtn for f, rtn in rules.items() if f(val))
        if result:
            return result
        else:
            return val
    return _joiner


rules = OrderedDict({
    modulo(3): 'fizz',
    modulo(5): 'buzz'
})

fizzbuzzer = joiner(rules)
