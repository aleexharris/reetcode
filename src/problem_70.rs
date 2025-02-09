// The recursive solution, but Leetcode runtime cuts out before you finish...
// pub fn climb_stairs(n: i32) -> i32 {
//     if n <= 3 {
//         n
//     } else {
//         Self::climb_stairs(n - 1) + Self::climb_stairs(n - 2)
//     }
// }


impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        (0..n).fold((1, 0), |(curr, prev), _idx| (curr + prev, curr)).0
    }
}