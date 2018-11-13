mod max_prime;

#[cfg(test)]
mod tests {

    use crate::max_prime::go;

    #[test]
    fn it_works_with_15() {
        assert_eq!(go(15), 5);
    }

    #[test]
    fn it_works_with_600851475143() {
        assert_eq!(go(600851475143), 6857)
    }

    #[test]
    fn it_works_with_25698751364526() {
        assert_eq!(go(25698751364526), 328513);
    }
}
