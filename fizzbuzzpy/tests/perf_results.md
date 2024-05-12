# Comparing multiple calls to rust, python native and single call to rust

```py
# ruff: noqa
import timeit

from fizzbuzzo3 import fizzbuzz as fbo3
from fizzbuzzpy import fizzbuzz as fbpy


REPEAT = 1
NUMBER = 10
FIZZBUZZES = 1_000_000
LISTOFNUMBERS = list(range(1, FIZZBUZZES))


def main():
    fbo3timer = timeit.Timer(stmt="[fbo3(i) for i in range(1,FIZZBUZZES)]", globals=globals())
    fbpytimer = timeit.Timer(stmt="[fbpy(i) for i in range(1,FIZZBUZZES)]", globals=globals())
    fbo3vectimer = timeit.Timer(stmt="[fbo3(LISTOFNUMBERS)]", globals=globals())
    print(f"Rust: [{REPEAT} calls of {NUMBER} runs fizzbuzzing up to {FIZZBUZZES}]")
    fbo3times = fbo3timer.repeat(repeat=REPEAT, number=NUMBER)
    print(fbo3times)
    print(f"Python: [{REPEAT} calls of {NUMBER} runs fizzbuzzing up to {FIZZBUZZES}]")
    fbpytimes = fbpytimer.repeat(repeat=REPEAT, number=NUMBER)
    print(fbpytimes)
    print(f"Rust vector: [{REPEAT} calls of {NUMBER} runs fizzbuzzing a list of numbers up to {FIZZBUZZES}]")
    fbo3vectimes = fbo3vectimer.repeat(repeat=REPEAT, number=NUMBER)
    print(fbo3vectimes)

if __name__ == "__main__":
    main()
```

## Without parallelisation

```text
Rust: [1 calls of 10 runs fizzbuzzing up to 1000000]
[12.454665303001093]
Python: [1 calls of 10 runs fizzbuzzing up to 1000000]
[39.32552230800138]
Rust vector: [1 calls of 10 runs fizzbuzzing a list of numbers up to 1000000]
[6.319926773001498]
```

## With parallelisation

Using up to 4 cores via rust ...

```text
Rust: [1 calls of 10 runs fizzbuzzing up to 1000000]
[12.806449372001225]
Python: [1 calls of 10 runs fizzbuzzing up to 1000000]
[39.52900022100221]
Rust vector: [1 calls of 10 runs fizzbuzzing a list of numbers up to 1000000]
[5.022411942001781]
```

```text
Rust: [1 calls of 10 runs fizzbuzzing up to 2000000]
[26.059991252001055]
Python: [1 calls of 10 runs fizzbuzzing up to 2000000]
[79.7828568210025]
Rust vector: [1 calls of 10 runs fizzbuzzing a list of numbers up to 2000000]
[9.551074810999125]
```
