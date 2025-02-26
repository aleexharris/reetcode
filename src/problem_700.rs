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
use std::cmp::Ordering;


impl Solution {
    pub fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {

        // The second let binding here wasn't obvious...
        // Similarly, Rc::clone() versus .clone() not obvious
        let root = root?;
        let node = root.borrow();
        
        match node.val.cmp(&val) {
            Ordering::Equal => Some(Rc::clone(&root)),
            Ordering::Greater => Self::search_bst(node.left.clone(), val),
            Ordering::Less => Self::search_bst(node.right.clone(), val),
        }
    }
}
