pub fn fizzbuzz(number: usize) -> String {
    let response = number.to_string();
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
}
