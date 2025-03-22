use std::cmp::Ordering;
use std::collections::HashSet;
use std::iter;

pub const SIZE: usize = 41;

impl Solution {
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort_unstable();
        let mut solution: [i32; SIZE] = [0; SIZE];
        let mut answer: HashSet<[i32; SIZE]> = HashSet::new();
        Self::combi_inner(candidates.as_slice(), &target, &mut solution, &mut answer);
        answer.iter().map(|b| Self::buckets_to_vec(b.into())).collect()
    }

    // For example: [0, 2, 0, 0, 3, 1] -> [1, 1, 4, 4, 4, 5]
    fn buckets_to_vec(bucket: &[i32; SIZE]) -> Vec<i32> {
        bucket
            .iter()
            .enumerate()
            .fold(Vec::new(), |mut acc, (i, qty)| {
                acc.extend(iter::repeat(i as i32).take(*qty as usize));
                acc
            })
    }

    fn combi_inner(
        candidates: &[i32],
        target: &i32,
        solution: &mut [i32; SIZE],
        answer: &mut HashSet<[i32; SIZE]>,
    ) {
        match 0_i32.cmp(target) {
            Ordering::Greater => (),
            Ordering::Equal => {
                answer.insert((*solution).into());
            },
            Ordering::Less => {
                let upper = candidates.partition_point(|i| i <= &target);
                for int in candidates[..upper].iter() {
                    solution[*int as usize] += 1;
                    Self::combi_inner(candidates, &(target - int), solution, answer);
                    solution[*int as usize] -= 1;
                }
            }
        };
    }
}
