struct Solution;
impl Solution {
  pub fn has_alternating_bits(n: i32) -> bool {
    // 01010101 -> 00101010 -> 01111111 -> 10000000 & 01111111 == 0
    // 10101010 -> 01010101 -> 1111111 -> 100000000 & 01111111 === 0
    let a = n ^ (n >> 1);
    return a & a + 1 == 0;
  }
}

fn main() {
  assert_eq!(Solution::has_alternating_bits(5), true);
  assert_eq!(Solution::has_alternating_bits(7), false);
  assert_eq!(Solution::has_alternating_bits(11), false);
}
