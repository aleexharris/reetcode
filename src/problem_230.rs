use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut stack = Vec::new();
        Self::dfs(root, &mut stack, k);
        stack.pop().expect("n > 0")
    }

    pub fn dfs(root: Option<Rc<RefCell<TreeNode>>>, stack: &mut Vec<i32>, k: i32) {
        if (stack.len() as i32) >= k {
            return;
        }
        if let Some(node) = root {
            let node = node.borrow();
            Self::dfs(node.left.clone(), stack, k);
            if (stack.len() as i32) < k {
                stack.push(node.val);
            }
            Self::dfs(node.right.clone(), stack, k);
        }
    }
}

