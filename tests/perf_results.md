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

## Passing a slice (as a representative of a range) vs a list

```text
Rust: [3 calls of 10 runs fizzbuzzing up to 1000000]
[13.941677560000244, 12.671054376998654, 12.669853160998173]
Rust vector: [3 calls of 10 runs fizzbuzzing a list of numbers up to 1000000]
[5.104824486003054, 4.96210950999739, 4.903727466000419]
Rust vector, with python list overhead: [3 calls of 10 runs creating and fizzbuzzing a list of numbers up to 1000000]
[5.363066075999086, 5.316481181002018, 5.361383773997659]
Rust range: [3 calls of 10 runs fizzbuzzing a range of numbers up to 1000000]
[3.8294942710017494, 3.8227306799999496, 3.800879727001302]
```

## Optimised build with `--release`

```text
Rust: [1 calls of 10 runs fizzbuzzing up to 1000000]
[3.3851599449990317]
Python: [1 calls of 10 runs fizzbuzzing up to 1000000]
[41.35110417800024]
Rust vector: [1 calls of 10 runs fizzbuzzing a list of numbers up to 1000000]
[0.8205389319991809]
Rust vector, with python list overhead: [1 calls of 10 runs creating and fizzbuzzing a list of numbers up to 1000000]
[1.1801474099993357]
Rust range: [1 calls of 10 runs fizzbuzzing a range of numbers up to 1000000]
[0.5420241989995702]
```

## Comparing return types (`-> str | list[str]` vs. `-> str`)

```text
(.venv) pyo3@6195c4a7706f:/workspaces/FizzBuzz$ echo "No LTO, Union"
No LTO, Union
(.venv) pyo3@6195c4a7706f:/workspaces/FizzBuzz$ /workspaces/FizzBuzz/.venv/bin/python /workspaces/FizzBuzz/tests/perftest.py
Rust: [1 calls of 10 runs fizzbuzzing up to 1000000]
[2.7247621990100015]
Rust vector: [1 calls of 10 runs fizzbuzzing a list of numbers up to 1000000]
[1.4409539260086603]
Rust vector, with python list overhead: [1 calls of 10 runs creating and fizzbuzzing a list of numbers up to 1000000]
[1.748141026997473]
Rust range: [1 calls of 10 runs fizzbuzzing a range of numbers up to 1000000]
[1.140464444004465]
(.venv) pyo3@6195c4a7706f:/workspaces/FizzBuzz$ echo "thin LTO, Union"
thin LTO, Union
(.venv) pyo3@6195c4a7706f:/workspaces/FizzBuzz$ /workspaces/FizzBuzz/.venv/bin/python /workspaces/FizzBuzz/tests/perftest.py
Rust: [1 calls of 10 runs fizzbuzzing up to 1000000]
[2.573878561001038]
Rust vector: [1 calls of 10 runs fizzbuzzing a list of numbers up to 1000000]
[1.5258527039986802]
Rust vector, with python list overhead: [1 calls of 10 runs creating and fizzbuzzing a list of numbers up to 1000000]
[1.7503311760083307]
Rust range: [1 calls of 10 runs fizzbuzzing a range of numbers up to 1000000]
[1.1543225019995589]
(.venv) pyo3@6195c4a7706f:/workspaces/FizzBuzz$ echo "fat LTO, Union"
fat LTO, Union
(.venv) pyo3@6195c4a7706f:/workspaces/FizzBuzz$ /workspaces/FizzBuzz/.venv/bin/python /workspaces/FizzBuzz/tests/perftest.py
Rust: [1 calls of 10 runs fizzbuzzing up to 1000000]
[2.659256437997101]
Rust vector: [1 calls of 10 runs fizzbuzzing a list of numbers up to 1000000]
[1.4467686470015906]
Rust vector, with python list overhead: [1 calls of 10 runs creating and fizzbuzzing a list of numbers up to 1000000]
[1.6921475639974233]
Rust range: [1 calls of 10 runs fizzbuzzing a range of numbers up to 1000000]
[1.1023815070075216]
(.venv) pyo3@6195c4a7706f:/workspaces/FizzBuzz$ echo "no LTO, String"
no LTO, String
(.venv) pyo3@6195c4a7706f:/workspaces/FizzBuzz$ /workspaces/FizzBuzz/.venv/bin/python /workspaces/FizzBuzz/tests/perftest.py
Rust: [1 calls of 10 runs fizzbuzzing up to 1000000]
[2.6100861899903975]
Rust vector: [1 calls of 10 runs fizzbuzzing a list of numbers up to 1000000]
[0.8597368839982664]
Rust vector, with python list overhead: [1 calls of 10 runs creating and fizzbuzzing a list of numbers up to 1000000]
[1.1903306849999353]
Rust range: [1 calls of 10 runs fizzbuzzing a range of numbers up to 1000000]
[0.6246530729986262]
```

## Comparing return types in general (1..10_000_000)

`Str`ings are 2x faster than `list`s created from `Vec<String>`

### `-> str`

```text
Rust: [1 calls of 10 runs fizzbuzzing up to 10000000]
[27.814233318000333]
Rust vector: [1 calls of 10 runs fizzbuzzing a list of numbers up to 10000000]
[7.261143727999297]
Rust vector, with python list overhead: [1 calls of 10 runs creating and fizzbuzzing a list of numbers up to 10000000]
[10.321640708003542]
Rust range: [1 calls of 10 runs fizzbuzzing a range of numbers up to 10000000]
[4.721871253001154]
```

### `-> str | list[str]`

```text
Rust: [1 calls of 10 runs fizzbuzzing up to 10000000]
[25.37807360100851]
Rust vector: [1 calls of 10 runs fizzbuzzing a list of numbers up to 10000000]
[12.790041114989435]
Rust vector, with python list overhead: [1 calls of 10 runs creating and fizzbuzzing a list of numbers up to 10000000]
[16.75132880899764]
Rust range: [1 calls of 10 runs fizzbuzzing a range of numbers up to 10000000]
[9.89638055099931]
```

### `-> list[str]`

```text
Rust: [1 calls of 10 runs fizzbuzzing up to 10000000]
[47.682113279995974]
Rust vector: [1 calls of 10 runs fizzbuzzing a list of numbers up to 10000000]
[12.776051424996695]
Rust vector, with python list overhead: [1 calls of 10 runs creating and fizzbuzzing a list of numbers up to 10000000]
[16.503915808003512]
Rust range: [1 calls of 10 runs fizzbuzzing a range of numbers up to 10000000]
[9.859676376989228]
```
