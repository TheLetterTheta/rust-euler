// Nicholas Dolan
// 2018-11-13
// Euler project #3
pub fn go(mut n: u64) -> u32 {
    let mut max_prime: u64 = 0;

    while n % 2 == 0 {
        max_prime = 2;
        n >>= 1;
    }

    let mut i: u64 = 3;
    while (i * i) <= n {
        while n % i == 0 {
            max_prime = i;
            n = n / i;
        }

        i += 2;
    }

    if n > 2 {
        max_prime = n;
    }
    return max_prime as u32;
}
