pub fn fizzbuzz(number: usize) -> String {
    let response: String;
    if number % 15 == 0 {
        response = "fizzbuzz".to_string();
    } else if number % 3 == 0 {
        response = "fizz".to_string();
    } else if number % 5 == 0 {
        response = "buzz".to_string();
    }
    else {
        response = number.to_string();
    }
    response
}

#[cfg(test)]
mod dumbtests {
    use super::*;

    #[test]
    fn one() {
        let result = fizzbuzz(1);
        assert_eq!(result, "1");
    }
    #[test]
    fn three() {
        let result = fizzbuzz(3);
        assert_eq!(result, "fizz");
    }

    #[test]
    fn six() {
        let result = fizzbuzz(6);
        assert_eq!(result, "fizz");
    }

    #[test]
    fn five() {
        let result = fizzbuzz(5);
        assert_eq!(result, "buzz");
    }
    #[test]
    fn fifteen() {
        let result = fizzbuzz(15);
        assert_eq!(result, "fizzbuzz");
    }
}
#[cfg(test)]
mod itertests {
    use super::*;

    #[test]
    fn threes() {
        for num in (0..=300).step_by(3) {
            let result = fizzbuzz(num);
            assert!(result == "fizz" || result == "fizzbuzz");
        }
    }

    #[test]
    fn five_is_buzz_or_fizzbuzz() {
        for num in (0..=300).step_by(5) {
            let result = fizzbuzz(num);
            assert!(result == "buzz" || result == "fizzbuzz");
        }
    }
}