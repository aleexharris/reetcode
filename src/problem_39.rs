use std::cmp::Ordering;

impl Solution {
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort_unstable();
        let mut solution: Vec<i32> = Vec::new();
        let mut answer: Vec<Vec<i32>> = Vec::new();
        Self::combi_inner(candidates.as_slice(), &target, &mut solution, &mut answer);
        answer
    }

    fn combi_inner(
        candidates: &[i32],
        target: &i32,
        solution: &mut Vec<i32>,
        answer: &mut Vec<Vec<i32>>,
    ) {
        match 0_i32.cmp(target) {
            Ordering::Greater => (),
            Ordering::Equal => {
                answer.push(solution.to_vec());
            },
            Ordering::Less => {
                let last = *solution.last().unwrap_or(&0_i32);
                candidates.iter().filter(|i| i <= &target && i >= &&last).for_each(|int| {
                    solution.push(*int);
                    Self::combi_inner(candidates, &(target - int), solution, answer);
                    solution.pop();
                });
            }
        }
    }
}
