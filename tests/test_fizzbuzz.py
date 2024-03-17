from fizzbuzz import fizzbuzzer


def test_lazy():
    assert fizzbuzzer(1) == 1
    assert fizzbuzzer(2) == 2
    assert fizzbuzzer(3) == "fizz"
    assert fizzbuzzer(4) == 4
    assert fizzbuzzer(5) == "buzz"
    assert fizzbuzzer(6) == "fizz"
    assert fizzbuzzer(15) == "fizzbuzz"


def test_rules():
    results = [fizzbuzzer(i) for i in range(1, 101)]
    OnlyIntOrFizzBuzz = all(r in ("fizz", "buzz", "fizzbuzz")  # noqa: N806
                            or isinstance(r, int) for r in results)
    assert OnlyIntOrFizzBuzz
    Every3rdHasFizz = all("fizz" in r for r in results[2::3])  # noqa: N806
    assert Every3rdHasFizz
    Every5thHasBuzz = all("buzz" in r for r in results[4::5])  # noqa: N806
    assert Every5thHasBuzz
    Every15thIsFizzBuzz = all(r == "fizzbuzz" for r in results[14::15])  # noqa: N806
    assert Every15thIsFizzBuzz
    EveryFizzIsMod3 = all((i+1) % 3 == 0 for i,  # noqa: N806
                          r in enumerate(results) if r == "fizz")
    assert EveryFizzIsMod3
    EveryBuzzIsMod5 = all((i+1) % 5 == 0 for i,  # noqa: N806
                          r in enumerate(results) if r == "buzz")
    assert EveryBuzzIsMod5
    EveryFizzBuzzIsMod15 = all(  # noqa: N806
        (i+1) % 15 == 0 for i, r in enumerate(results) if r == "fizzbuzz")
    assert EveryFizzBuzzIsMod15
    AllIntInPlace = all(  # noqa: N806
        (i+1) == r for i, r in enumerate(results) if isinstance(r, int))
    assert AllIntInPlace
