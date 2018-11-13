use num::integer::gcd;

// Nicholas Dolan
// 2018-11-13
// Euler project #5
pub fn go(nums: Vec<u32>) -> u64 {
    let mut lcm: u64 = nums[0] as u64;

    for i in nums {
        lcm = ((i as u64) * lcm)/gcd(i as u64, lcm);
    }
    lcm
}
