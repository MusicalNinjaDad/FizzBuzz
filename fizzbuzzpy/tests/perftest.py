# ruff: noqa
import timeit

from fizzbuzzo3 import fizzbuzz as fbo3
from fizzbuzzpy import fizzbuzz as fbpy


REPEAT = 5
NUMBER = 10
FIZZBUZZES = 100_000

def main():
    fbo3timer = timeit.Timer(stmt="[fbo3(i) for i in range(1,FIZZBUZZES)]", globals=globals())
    fbpytimer = timeit.Timer(stmt="[fbpy(i) for i in range(1,FIZZBUZZES)]", globals=globals())
    print(f'Rust: [{REPEAT} calls of {NUMBER} runs fizzbuzzing up to {FIZZBUZZES}]')
    fbo3times = fbo3timer.repeat(repeat=REPEAT, number=NUMBER)
    print(fbo3times)
    print(f'Python: [{REPEAT} calls of {NUMBER} runs fizzbuzzing up to {FIZZBUZZES}]')
    fbpytimes= fbpytimer.repeat(repeat=REPEAT, number=NUMBER)
    print(fbpytimes)

if __name__ == '__main__':
    main()