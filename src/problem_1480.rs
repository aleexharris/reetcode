impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        nums.iter().scan(0, |acc, &n| {
            *acc += n;
            Some(*acc)
        }).collect()
    }
}
