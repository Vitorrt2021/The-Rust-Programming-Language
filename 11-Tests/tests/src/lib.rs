pub fn add(left: usize, right: usize) -> usize {
    left + right
}

fn multiply(left: usize, right: usize) -> usize {
    left * right
}

fn be_equal(left: usize, right: usize) -> bool {
    left == right
}

fn get_panic(){
    panic!("Failed to get panic");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[ignore]
    fn should_multiply() {
        let result = multiply(2,3);
        assert_eq!(result, 6);
    }

    #[test]
    fn should_check_equal(){
        assert!(be_equal(2,2));
    }

    #[test]
    #[should_panic]
    fn should_panic() {
        get_panic()
    }

    #[test]
    #[should_panic(expected = "Failed to get panic")]
    fn should_panic_with_text() {
        get_panic()
    }
}
