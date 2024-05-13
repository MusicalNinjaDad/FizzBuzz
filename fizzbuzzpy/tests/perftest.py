# ruff: noqa
import timeit

from fizzbuzzo3 import fizzbuzz as fbo3
from fizzbuzzpy import fizzbuzz as fbpy


REPEAT = 1
NUMBER = 10
FIZZBUZZES = 100_000
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
