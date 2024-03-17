from fizzbuzz import fizzbuzzer


def test_lazy():
    assert fizzbuzzer(1) == "1"
    assert fizzbuzzer(2) == "2"
    assert fizzbuzzer(3) == "fizz"
    assert fizzbuzzer(4) == "4"
    assert fizzbuzzer(5) == "buzz"
    assert fizzbuzzer(6) == "fizz"
    assert fizzbuzzer(15) == "fizzbuzz"


def test_rules():
    results = [fizzbuzzer(i) for i in range(1, 101)]
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
