// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
use std::cell::Ref;
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        let mut path: Vec<i32> = Vec::new();
        Self::path_target_sum(root, &mut path, &target_sum)
    }
    
    // DFS backtracking-style brute force algorithm
    // First check if current node is success, then check the children for success
    // Building path is most literal solution, but not simplest solution
    // Simpler: take away the value of the current node from the target
    // eg. return (target - node.val == 0.0)
    // true => path sums to target, false => path does not sum to target
    pub fn path_target_sum(root: Option<Rc<RefCell<TreeNode>>>, path: &mut Vec<i32>, target: &i32) -> bool {
        let mut result = false;
        if let Some(root) = root {
            let node = root.borrow();
            path.push(node.val);
            if (node.left.is_none() && node.right.is_none()) {
                result = path.iter().sum::<i32>() == *target;
            } else {
                result = (
                    Self::path_target_sum(node.left.clone(), path, target) 
                    || Self::path_target_sum(node.right.clone(), path, target)
                );
            }
            path.pop();
        }
        result
    }
}
