use fizzbuzz::MultiFizzBuzz;

mod vectors {

    use super::*;

    #[test]
    fn test_vec() {
        let mut input = vec![1, 2, 3, 4, 5];
        let answer: Vec<String> = input.fizzbuzz().into();
        let expected = vec![
            "1".to_string(),
            "2".to_string(),
            "fizz".to_string(),
            "4".to_string(),
            "buzz".to_string(),
        ];
        assert_eq!(answer, expected)
    }
}