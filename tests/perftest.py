# ruff: noqa
import timeit

from fizzbuzz.fizzbuzzo3 import fizzbuzz as fbo3
from fizzbuzz.fizzbuzzpy import fizzbuzz as fbpy


REPEAT = 1
NUMBER = 10
FIZZBUZZES = 10_000_000
LISTOFNUMBERS = list(range(1, FIZZBUZZES))


def main():

    print(f"Rust: [{REPEAT} calls of {NUMBER} runs fizzbuzzing up to {FIZZBUZZES}]")
    fbo3timer = timeit.Timer(stmt="[fbo3(i) for i in range(1,FIZZBUZZES)]", globals=globals())
    fbo3times = fbo3timer.repeat(repeat=REPEAT, number=NUMBER)
    print(fbo3times)
    
    # print(f"Python: [{REPEAT} calls of {NUMBER} runs fizzbuzzing up to {FIZZBUZZES}]")
    # fbpytimer = timeit.Timer(stmt="[fbpy(i) for i in range(1,FIZZBUZZES)]", globals=globals())
    # fbpytimes = fbpytimer.repeat(repeat=REPEAT, number=NUMBER)
    # print(fbpytimes)
    
    print(f"Rust vector: [{REPEAT} calls of {NUMBER} runs fizzbuzzing a list of numbers up to {FIZZBUZZES}]")
    fbo3vectimer = timeit.Timer(stmt="[fbo3(LISTOFNUMBERS)]", globals=globals())
    fbo3vectimes = fbo3vectimer.repeat(repeat=REPEAT, number=NUMBER)
    print(fbo3vectimes)

    print(f"Rust vector, with python list overhead: [{REPEAT} calls of {NUMBER} runs creating and fizzbuzzing a list of numbers up to {FIZZBUZZES}]")
    fbo3vecplustimer = timeit.Timer(stmt="[fbo3(list(range(1, FIZZBUZZES)))]", globals=globals())
    fbo3vecplustimes = fbo3vecplustimer.repeat(repeat=REPEAT, number=NUMBER)
    print(fbo3vecplustimes)

    print(f"Rust range: [{REPEAT} calls of {NUMBER} runs fizzbuzzing a range of numbers up to {FIZZBUZZES}]")
    fbo3rangetimer = timeit.Timer(stmt="[fbo3(slice(1, FIZZBUZZES))]", globals=globals())
    fbo3rangetimes = fbo3rangetimer.repeat(repeat=REPEAT, number=NUMBER)
    print(fbo3rangetimes)

if __name__ == "__main__":
    main()
