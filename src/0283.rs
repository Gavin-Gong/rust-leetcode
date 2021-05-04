struct Solution;
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut i = 0;
        let mut j = 0;
        while j < nums.len() {
            if nums[j] != 0 {
                nums[i] = nums[j];
                i += 1;
            }
            j= j + 1
        }
        while i < nums.len() {
            nums[i] = 0;
            i += 1;
        }

    }
}


fn main() {
    let mut vec: Vec<i32> = vec![0, 1, 2, 3, 0];
    Solution::move_zeroes(&mut vec);
    println!("{:?}", vec);
}
