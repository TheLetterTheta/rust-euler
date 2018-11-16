mod even_fibonacci;

#[cfg(test)]
mod tests {
    use crate::even_fibonacci::go;

    #[test]
    fn it_works_with_4() {
        assert_eq!(go(4), 188);
    }

    #[test]
    fn it_works_with_10() {
        assert_eq!(go(10), 1089154);
    }

    #[test]
    fn it_works_with_20() {
        assert_eq!(go(20), 4613732);
    }

    #[test]
    fn it_works_with_300_because_overflow() {
        assert_eq!(go(300), 4613732);
    }
}
