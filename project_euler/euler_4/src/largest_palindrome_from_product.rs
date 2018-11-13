// Nicholas Dolan
// 2018-11-13
// Euler project #4
pub fn go(num_digits: u8) -> u32 {
    let product: u32 = 10_u32.pow(num_digits as u32) - 1;

    for i in (0..product + 1).rev() {

        for j in (i..product + 1).rev() {

            if is_palindrome(i * j){
                return i * j;
            }

        }

    }

    0
}

fn is_palindrome(num: u32) -> bool {
    let num_string = num.to_string();
    let half = num_string.len() / 2;

    num_string.bytes().take(half).eq(num_string.bytes().rev().take(half))
}
