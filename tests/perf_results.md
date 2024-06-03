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

## Passing a slice (as a representative of a range) vs a list:

Rust: [3 calls of 10 runs fizzbuzzing up to 1000000]
[13.941677560000244, 12.671054376998654, 12.669853160998173]
Rust vector: [3 calls of 10 runs fizzbuzzing a list of numbers up to 1000000]
[5.104824486003054, 4.96210950999739, 4.903727466000419]
Rust vector, with python list overhead: [3 calls of 10 runs creating and fizzbuzzing a list of numbers up to 1000000]
[5.363066075999086, 5.316481181002018, 5.361383773997659]
Rust range: [3 calls of 10 runs fizzbuzzing a range of numbers up to 1000000]
[3.8294942710017494, 3.8227306799999496, 3.800879727001302]