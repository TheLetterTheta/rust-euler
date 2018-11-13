mod even_fibonacci;

#[cfg(test)]
mod tests {
    use crate::even_fibonacci::go;

    #[test]
    fn it_works_with_4() {
        assert_eq!(go(4), 2);
    }

    #[test]
    fn it_works_with_10() {
        assert_eq!(go(10), 44);
    }

    #[test]
    fn it_works_with_20() {
        assert_eq!(go(20), 3382);
    }

    #[test]
    fn it_works_with_30() {
        assert_eq!(go(30), 1089154);
    }

    #[test]
    fn it_works_with_32() {
        assert_eq!(go(32), 1089154);
    }

    #[test]
    fn it_works_with_33_last_overflow() {
        assert_eq!(go(33), 4613732);
    }

    #[test]
    fn it_works_with_300_because_overflow() {
        assert_eq!(go(300), 4613732);
    }
}
