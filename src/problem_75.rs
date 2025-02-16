impl Solution {
    // Iter fold is nice for bucket sort
    // Note the requirement to `mut acc` to increment from iter.
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut counts = nums.iter().fold([0;3], |mut acc, &val| {
            acc[val as usize] += 1;
            acc
        });
        let mut i: usize = 0;
        for (colour, &count) in counts.iter().enumerate() {
            for _ in 0..count {
                nums[i] = colour as i32;
                i += 1;
            } 
        }
    }
}
