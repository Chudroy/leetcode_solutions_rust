use crate::models::Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut count = 0;
        for i in 1..nums.len() {
            if nums[i] == nums[i - 1] {
                count += 1;
            } else {
                nums[i - count] = nums[i];
            }
        }

        (nums.len() - count) as i32
    }
}
