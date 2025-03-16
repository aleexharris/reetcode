use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::Ordering;
impl Solution {
    // If you are returning an Rc::clone(), try instead to drop() the borrow before returning the Rc
    // If you need to change a parent reference from a recursive fn, instead try changing the child value and the fn's target value.
    // .take() is more efficient than .clone() but be careful not to "double take" in the same code branch
    pub fn find_max(subtree: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let node = subtree?;
        let node_ref = node.borrow();
        if node_ref.right.is_none() {
            drop(node_ref);
            return Some(node)
        }
        return Self::find_max(node_ref.right.clone())
    }

    pub fn delete_node(root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
        let root = root?;
        let mut node = root.borrow_mut();
        match key.cmp(&node.val) {
            Ordering::Less => {
                node.left = Self::delete_node(node.left.take(), key);
            },
            Ordering::Greater => {
                node.right = Self::delete_node(node.right.take(), key);
            },
            Ordering::Equal => {
                if node.left.is_none() {
                    return node.right.take()
                } else {
                    if let Some(predecessor) = Self::find_max(node.left.clone()) {
                        let val = predecessor.borrow().val;
                        node.val = val;
                        node.left = Self::delete_node(node.left.take(), val);
                    }
                }
            },
        };
        drop(node);
        Some(root)
    }
}
