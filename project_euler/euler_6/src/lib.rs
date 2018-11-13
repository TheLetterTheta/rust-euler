mod sum_square_difference;

#[cfg(test)]
mod tests {

    use crate::sum_square_difference::go;

    #[test]
    fn it_works_for_10() {
        assert_eq!(go(10), 2640);
    }

    #[test]
    fn it_works_for_100() {
        assert_eq!(go(100), 25164150);
    }
}
