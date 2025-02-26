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

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut vals = Vec::new();
        Self::depth_first_search(root, &mut vals);
        vals
    }

    // Easy to realise that you need a helper function, but didn't realise that was allowed...
    // Took me a second longer to realise you also need a &mut to the output data structure.
    pub fn depth_first_search(root: Option<Rc<RefCell<TreeNode>>>, vals: &mut Vec<i32>) {
        if let Some(root) = root {
            let node = root.borrow();
            Self::depth_first_search(node.left.clone(), vals);
            vals.push(node.val);
            Self::depth_first_search(node.right.clone(), vals);
        }
    }
}
