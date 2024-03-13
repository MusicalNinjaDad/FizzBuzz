pub fn fizzbuzz(number: usize) -> String {
    let response: String;
    if number == 3 {
        response = "fizz".to_string();
    } else {
        response = number.to_string();
    }
    response
}

#[cfg(test)]
mod tests {
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
}
