use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;


// use if let Some(x) = option.take() pattern matching more
// use mut on input root Option<T> as this allows you to use the above technique on the input
impl Solution {
    pub fn right_side_view(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut output_vec = Vec::new();
        if let Some(node) = root.take() {
            let mut queue = VecDeque::from(vec![node]);
            while queue.len() > 0 {
                if let Some(b) = queue.front() {
                    output_vec.push(b.borrow().val);
                }
                for _ in 0..queue.len() {
                    let parent = queue.pop_front().expect("queue.len() > 0");
                    let mut parent = parent.borrow_mut();
                    if let Some(child) = parent.right.take() {
                        queue.push_back(child);
                    }
                    if let Some(child) = parent.left.take() {
                        queue.push_back(child);
                    }
                }
            }
        }
        output_vec
    }
}
