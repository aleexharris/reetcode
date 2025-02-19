impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        nums.iter().enumerate().map(|(i, _)| nums[..=i].iter().sum()).collect()
    }
}
