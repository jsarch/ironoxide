pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, add_two(2));
    }
}

#[cfg(test)]
mod more_tests {
    use super::*;

    #[test]
    #[should_panic]
    fn it_doesnt_work() {
        assert_eq!(4, add_two(3));
    }
}
