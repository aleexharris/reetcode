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
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    // Rejigged their rubbish choice of data structures for inputs
    // Recursion => What's the base case? Probably something null.
    // Each val from the preorder array is the root node of a subtree in the inorder array
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut root_queue = VecDeque::from(preorder);
        Self::recursive_build(&mut root_queue, inorder.as_slice())
    }

    pub fn recursive_build(root_queue: &mut VecDeque<i32>, subtree: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if subtree.len() == 0 {
            return None;
        }
        let root_val = root_queue.pop_front()?;
        let root_i = subtree.iter().position(|x| *x == root_val)?;
        let left = Self::recursive_build(root_queue, &subtree[..root_i]);
        let right = Self::recursive_build(root_queue, &subtree[root_i + 1..]);
        Some(Rc::new(RefCell::new(TreeNode {
            val: root_val,
            left,
            right,
        })))
    }
}

