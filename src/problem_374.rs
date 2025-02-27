/** 
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is higher than the picked number
 *			      1 if num is lower than the picked number
 *               otherwise return 0
 * unsafe fn guess(num: i32) -> i32 {}
 */

use std::cmp::Ordering;

impl Solution {
    // Easy enough. Ordering Enum can be used the make match statement exhaustive.
    unsafe fn guessNumber(n: i32) -> i32 {
        let mut mid;
        let (mut left, mut right) = (1, n);
        loop {
            mid = left + (right - left) / 2;
            match guess(mid).cmp(&0_i32) {
                Ordering::Less => right = mid - 1,
                Ordering::Greater => left = mid + 1,
                Ordering::Equal => return mid,
            }
            
        }
        
    }
}
