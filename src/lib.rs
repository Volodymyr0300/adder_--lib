pub fn add_two(a: i32) -> i32 { 
    a + 2 
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test] // cargo test add
    fn add_two_and_two() {
        assert_eq!(4, add_two(2))
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3))
    }

    #[test] // cargo test one_hundred
    fn one_hundred() {
        assert_eq!(102, add_two(100))
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4)
    }

    #[test]
    #[ignore] // cargo test -- --ignored
    fn expensive_test() {
        // code that takes an hour to run
    }
}