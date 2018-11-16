// Nicholas Dolan
// 2018-11-13
// Euler project #7

pub fn go(n: u32) -> u64{
    nth_prime(n)
}

fn nth_prime(n: u32) -> u64 {

    if n == 1 {
        return 2;
    }

    let mut curr_prime: u64 = 3;

    for _i in 2..n {
        curr_prime += 2;
        while !is_prime(curr_prime){
            curr_prime += 2;
        }
    }
    curr_prime
}

fn is_prime(n: u64) -> bool {
    for i in (3..((n as f64).sqrt() as u64) + 1).step_by(2){
        if n % i == 0 {
            return false
        }
    }
    true
}
