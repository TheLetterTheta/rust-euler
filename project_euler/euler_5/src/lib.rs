extern crate num;

mod least_common_multiple_set;

#[cfg(test)]
mod tests {

    use crate::least_common_multiple_set::go;

    #[test]
    fn it_works_for_10() {
        assert_eq!(go((1..10).collect()), 2520);
    }

    #[test]
    fn it_works_for_20() {
        assert_eq!(go((1..20).collect()), 232792560);
    }
}
