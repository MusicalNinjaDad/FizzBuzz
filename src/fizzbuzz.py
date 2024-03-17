from collections import OrderedDict  # noqa: INP001, D100
from typing import Any, Callable


def modulo(n: int) -> Callable[[int], bool]:  # noqa: D103
    def _modulo(x: int) -> bool:
        return x % n == 0
    return _modulo


def joiner(rules: dict) -> Callable[[Any], Any]:  # noqa: D103
    def _joiner(val):  # noqa: ANN001
        result = "".join(rtn for f, rtn in rules.items() if f(val))
        if result:
            return result
        else:  # noqa: RET505
            return val
    return _joiner


rules = OrderedDict({
    modulo(3): "fizz",
    modulo(5): "buzz"  # noqa: COM812
})

fizzbuzzer = joiner(rules)
