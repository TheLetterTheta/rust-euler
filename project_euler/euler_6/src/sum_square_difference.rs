// Nicholas Dolan
// 2018-11-13
// Euler project #6
pub fn go(n: u32) -> u64 {

    let mut sum_square: u64 = 0;
    let mut square_sum: u64 = 0;

    for i in 1..n+1 {
        sum_square += (i*i) as u64;
        square_sum += i as u64;
    }

    square_sum = square_sum * square_sum;

    square_sum - sum_square
}
