impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        // Edgecase
        if nums.len() == 0 {
            return 0;
        }

        // Slow pointer, fast pointer technique
        let mut slow = 0;
        println!("slow | fast");
        for fast in 1..nums.len() {
            println!("{:?}->{:?} | {:?}->{:?}", slow, nums[slow], fast, nums[fast]);
            if nums[fast] != nums[slow] {
                slow +=1;
                nums[slow] = nums[fast];
            }

        }
        (slow + 1) as i32
    }
}
