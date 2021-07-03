use std::collections::HashMap;

struct Solution;
impl Solution {
  pub fn frequency_sort(s: String) -> String {
    let mut map: HashMap<char, i32> = HashMap::new();
    for c in s.chars() {
      *map.entry(c).or_insert(0) += 1;
    }
    let mut v = map.iter().collect::<Vec<(&char, &i32)>>();
    v.sort_by(|a, b| b.1.cmp(a.1));

    return v.into_iter().fold(String::from(""), |acc, cur| {
      acc + &cur.0.to_string().repeat(*cur.1 as usize)
    });
  }
}

fn main() {
  assert_eq!(
    Solution::frequency_sort(String::from("aacsasa")),
    "aaaasscc"
  );
}
