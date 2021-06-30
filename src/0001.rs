use std::collections::HashMap;
struct Solution;
impl Solution {
  pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, usize> = HashMap::new();
    for i in 0..nums.len() {
      let r = target - nums[i];
      if map.contains_key(&r) {
        return vec![i as i32, *map.get(&r).unwrap() as i32]
      } else {
        map.insert(nums[i], i);
      }
    }
    vec![]
  }
}


fn main() {
  assert_eq!(Solution::two_sum(vec![2,7,11,15], 9), vec![1, 0])
}
