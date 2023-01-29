pub fn is_even(number: i64) -> bool {
    if number % 2 == 0 { true } else { false }
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(is_even(2), true);
        assert_eq!(is_even(3), false);
        assert_eq!(is_even(4), true);
        assert_eq!(is_even(5), false);
    }
}
