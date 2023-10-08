fn print_and_returns_10(a: i32) -> i32 {
    println!("I got the number {}", a);
    10
}

#[cfg(test)]
mod awesome_test {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = print_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = print_and_returns_10(4);
        assert_eq!(5, value);
    }
}
