mod largest_palindrome_from_product;

#[cfg(test)]
mod tests {

    use crate::largest_palindrome_from_product::go;

    #[test]
    fn it_works_with_2() {
        assert_eq!(go(2), 9009);
    }

    #[test]
    fn it_works_with_3() {
        assert_eq!(go(3), 906609);
    }
}
