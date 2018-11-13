// Nicholas Dolan
// 2018-11-13
// Project Euler #1
pub fn go(n: u16) -> u32 {

    let mut sum: u32 = 0;

    for i in 1..n {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i as u32;
        }
    }

    return sum;
}
