import pytest
from fizzbuzz.fizzbuzzo3 import fizzbuzz


def test_lazy():
    assert fizzbuzz(1) == "1"
    assert fizzbuzz(2) == "2"
    assert fizzbuzz(3) == "fizz"
    assert fizzbuzz(4) == "4"
    assert fizzbuzz(5) == "buzz"
    assert fizzbuzz(6) == "fizz"
    assert fizzbuzz(15) == "fizzbuzz"

def test_float():
    assert fizzbuzz(1.0) == "1"
    assert fizzbuzz(3.0) == "fizz"

def test_list():
    assert fizzbuzz([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 15]) == "1, 2, fizz, 4, buzz, fizz, 7, 8, fizz, buzz, fizzbuzz"

def test_string():
    with pytest.raises(TypeError):
        fizzbuzz("1")

def test_1_to_100():
    results = [fizzbuzz(i) for i in range(1, 101)]
    every_3rd_has_fizz = all("fizz" in r for r in results[2::3])
    assert every_3rd_has_fizz
    every_5th_has_buzz = all("buzz" in r for r in results[4::5])
    assert every_5th_has_buzz
    every_15th_is_fizzbuzz = all(r == "fizzbuzz" for r in results[14::15])
    assert every_15th_is_fizzbuzz
    every_fizz_is_mod3 = all((i + 1) % 3 == 0 for i, r in enumerate(results) if r == "fizz")
    assert every_fizz_is_mod3
    every_buzz_is_mod5 = all((i + 1) % 5 == 0 for i, r in enumerate(results) if r == "buzz")
    assert every_buzz_is_mod5
    every_fizzbuzz_is_mod15 = all((i + 1) % 15 == 0 for i, r in enumerate(results) if r == "fizzbuzz")
    assert every_fizzbuzz_is_mod15
    all_numbers_correct = all(r == str(i + 1) for i, r in enumerate(results) if r not in ("fizz", "buzz", "fizzbuzz"))
    assert all_numbers_correct

def test_slice():
     assert fizzbuzz(slice(1,16,1)) == "1, 2, fizz, 4, buzz, fizz, 7, 8, fizz, buzz, 11, fizz, 13, 14, fizzbuzz"

def test_slice_no_step():
     assert fizzbuzz(slice(1,16)) == "1, 2, fizz, 4, buzz, fizz, 7, 8, fizz, buzz, 11, fizz, 13, 14, fizzbuzz"

def test_slice_negative_step():
    assert fizzbuzz(slice(15,0,-3)) == "fizzbuzz, fizz, fizz, fizz, fizz"

def test_slice_zero_step():
    with pytest.raises(ValueError, match="step cannot be zero"):
        fizzbuzz(slice(1,16,0))