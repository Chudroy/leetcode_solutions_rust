use crate::models::Solution;

/**
 * 26. Remove Duplicates from Sorted Array
 * Given an integer array nums sorted in non-decreasing order,
 * remove the duplicates in-place such that each unique element appears only once. 
 * The relative order of the elements should be kept the same. Then return the number of unique elements in nums.
 * Consider the number of unique elements of nums to be k, to get accepted, you need to do the following things:
 * Change the array nums such that the first k elements of nums contain the unique elements in the order they were 
 * present in nums initially. The remaining elements of nums are not important as well as the size of nums.
 * Return k.
 */
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
