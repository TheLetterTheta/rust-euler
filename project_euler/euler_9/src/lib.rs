mod pythagorean_tripple;

#[cfg(test)]
mod tests {

    use crate::pythagorean_tripple::go;

    #[test]
    fn it_works_for_1000() {
        assert_eq!(go(1000), 31875000);
    }
}
