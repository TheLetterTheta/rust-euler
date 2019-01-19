extern crate num;
mod sum_of_primes;

#[cfg(test)]
mod tests {

    use crate::sum_of_primes::PrimeVect;

    #[test]
    fn it_works_for_1000() {
        let i: u64 = PrimeVect::<u64>::new()
            .take_while(|p| p < &2000000)
            .sum();
        println!("{}", i);
        assert_eq!(1,2)

    }
}
