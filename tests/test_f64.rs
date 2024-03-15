use fizzbuzz::f64;
use fizzbuzz::FizzBuzz;

#[test]
fn test_f64() {
    let allnums: Vec<f64> = (1i16..=300).step_by(1).map(|i| f64::from(i)).collect();
    let fifteens: Vec<f64> = (0i16..=300).step_by(15).map(|i| f64::from(i)).collect();
    let fives: Vec<f64> = (0i16..=300).step_by(5).map(|i| f64::from(i)).collect();
    let threes: Vec<f64> = (0i16..=300).step_by(3).map(|i| f64::from(i)).collect();

    for num in allnums {
        let result = num.fizzbuzz();
        if fifteens.contains(&num) {
            assert_eq!(result, "fizzbuzz")
        } else if fives.contains(&num) {
            assert_eq!(result, "buzz")
        } else if threes.contains(&num) {
            assert_eq!(result, "fizz")
        } else {
            assert_eq!(&result, &num.to_string())
        }
    }
}
