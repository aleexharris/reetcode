use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::Ordering;

impl Solution {
    pub fn insert_into_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        let root = root.unwrap_or_else(|| Rc::new(RefCell::new(TreeNode::new(val))));
        let mut node = root.borrow_mut();
        match val.cmp(&node.val) {
            Ordering::Less => {
                node.left = Self::insert_into_bst(node.left.take(), val);
            }
            Ordering::Greater => {
                node.right = Self::insert_into_bst(node.right.take(), val);
            }
            Ordering::Equal => (),
        }
        drop(node);
        Some(root)
    }
}
