mod sum_multiples_of_three_and_five;

#[cfg(test)]
mod tests {

    use crate::sum_multiples_of_three_and_five::go;

    #[test]
    fn it_works_with_10() {
        assert_eq!(go(10), 23)
    }

    #[test]
    fn it_works_with_1000() {
        assert_eq!(go(1000), 233168);
    }
}
