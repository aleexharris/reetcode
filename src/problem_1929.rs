impl Solution {
    // Too easy a question IMO
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        if (nums.len() == 0) {
            return Vec::new();
        }

        let mut ans = Vec::with_capacity(2 * nums.len());
        for _ in 0..2 {
            for i in 0..nums.len() {
                ans.push(nums[i]);
            };
        };
        ans
    }
}
