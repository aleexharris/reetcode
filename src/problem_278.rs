// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)

impl Solution {
    // Bog standard BST question.
    // Key insight is that when left == mid, this is the left-most bad version
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut mid;
        let (mut left, mut right) = (1, n);
        loop {
            mid = left + (right - left) / 2;
            if self.isBadVersion(mid) {
                if (&left == &mid) {
                    break left;
                }
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
    }
}
