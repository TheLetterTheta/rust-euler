// Nicholas Dolan
// 2018-11-13
// Euler project #4
pub fn go(num_digits: u8) -> u32 {
    let product: u32 = 10_u32.pow(num_digits as u32);

    (1..product)
        .flat_map(|i|
                  (i..product)
                  .map(move |j| j * i))
        .filter(|n| is_palindrome(*n))
        .max()
        .unwrap()
}

fn is_palindrome(num: u32) -> bool {
    let num_string = num.to_string();
    let half = num_string.len() / 2;

    num_string.bytes().take(half).eq(num_string.bytes().rev().take(half))
}
