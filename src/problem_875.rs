use std::cmp::Ordering;

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut left: i32 = 1;
        let mut right: i32 = *piles.iter().max().expect("Piles.len() >= 1");
        let base_acc = piles.len() as i32;
        while left < right {
            let mut mid = left + (right - left) / 2;
            let mut h_guess = piles.iter().fold(base_acc, |acc, x| acc + (x - 1) / mid);
            match h_guess.cmp(&h) {
                Ordering::Greater => left = mid + 1,
                _ => right = mid,
            }
        }
        right
    }
}
