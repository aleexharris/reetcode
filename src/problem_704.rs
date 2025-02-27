use std::cmp::Ordering;

impl Solution {
    // Always use the two pointer technique for array questions
    // When updating l/r pointers, you can go past mid, as you just checked it
    // usize was blowing up for len=0, so need to use i32 for arithmetic
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (0, nums.len() as i32 - 1);
        while left <= right {
            let mut mid = left + (right - left) / 2;
            match &target.cmp(&nums[mid as usize]) {
                Ordering::Equal => return mid as i32,
                Ordering::Greater => left = mid + 1,
                Ordering::Less => right = mid - 1,
            }
        }
        -1
    }
}
