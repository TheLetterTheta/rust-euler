// Nicholas Dolan
// 2018-11-13
// Project Euler #2
pub fn go(n: u32) -> u64 {

    let mut iter_prev: u32 = 1;
    let mut iter_curr: u32 = 2;
    let mut sum: u64 = 2;

    if n < 3 {
        return 0;
    }

    for _ in 3..n {

        let t: u32 = iter_curr;
        iter_curr = iter_prev + t;
        iter_prev = t;

        if iter_curr > 4000000 {
            break;
        }

        if iter_curr % 2 == 0{
            sum += iter_curr as u64;
        }

    }

    return sum;
}
