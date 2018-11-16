mod nth_prime;

#[cfg(test)]
mod tests {

    use crate::nth_prime::go;

    #[test]
    fn it_works_with_1() {
        assert_eq!(go(1), 2);
    }

    #[test]
    fn it_works_with_2() {
        assert_eq!(go(2), 3);
    }

    #[test]
    fn it_works_with_3() {
        assert_eq!(go(3), 5);
    }

    #[test]
    fn it_works_with_4() {
        assert_eq!(go(4), 7);
    }

    #[test]
    fn it_works_with_5() {
        assert_eq!(go(5), 11);
    }

    #[test]
    fn it_works_with_6() {
        assert_eq!(go(6), 13);
    }

    #[test]
    fn it_works_with_10001() {
        assert_eq!(go(10001), 104743);
    }
}
