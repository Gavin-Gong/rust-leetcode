struct Solution;
impl Solution {
  /// 递归
  pub fn num_ways(n: i32, relation: Vec<Vec<i32>>, k: i32) -> i32 {
    let mut count: i32 = 0;
    if k == 1 {
      for i in 0..relation.len() {
        if relation[i][0] == 0 && relation[i][1] == n - 1 {
          count += 1;
        }
      }
      return count;
    }

    for i in 0..relation.len() {
      if relation[i][1] == n - 1 {
        count += Solution::num_ways(relation[i][0] + 1, relation.clone(), k - 1);
      }
    }
    count
  }
}

fn main() {
  assert_eq!(
    Solution::num_ways(
      5,
      vec![
        vec![0, 2],
        vec![2, 1],
        vec![3, 4],
        vec![2, 3],
        vec![1, 4],
        vec![2, 0],
        vec![0, 4]
      ],
      3
    ),
    3
  );
  assert_eq!(Solution::num_ways(3, vec![vec![0, 2], vec![2, 1]], 2), 0);
}
