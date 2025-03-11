use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let dummy_node = Rc::new(RefCell::new(TreeNode{val: i32::MIN, left: root, right: None}));

        let mut rows = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_front(dummy_node);
        while queue.len() != 0 {
            let mut row_len = queue.len();
            let mut new_row = Vec::new();
            for i in 0..row_len {
                let curr = queue.pop_back().expect("Already checking if len > 0");
                let curr = curr.borrow();
                new_row.push(curr.val.clone());
                if i == row_len - 1 {
                    rows.push(new_row.clone());
                }
                if let Some(left) = &curr.left {
                    queue.push_front(Rc::clone(&left));
                }
                if let Some(right) = &curr.right {
                    queue.push_front(Rc::clone(&right));
                }
            }
        }
        rows.remove(0);
        rows
    }
}
