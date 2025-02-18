use std::collections::VecDeque;

struct MyStack {
    left: VecDeque<i32>,
    right: VecDeque<i32>,
    left_in: bool,
    first_out: bool,
}

impl MyStack {
    /** My "LIFO" stack haha **/

    fn new() -> Self {
        MyStack {
            left: VecDeque::new(),
            right: VecDeque::new(),
            left_in: true,
            first_out: true,
        }
        
    }
    
    fn push(&mut self, x: i32) {
        match self.left_in {
            true => {
                self.right.append(&mut self.left);
                self.left.push_back(x);
            },
            false => {
                self.left.append(&mut self.right);
                self.right.push_back(x);
            },
        };
        self.left_in = !self.left_in;
        self.first_out = true;
    }
    
    fn pop(&mut self) -> i32 {
        let x = match self.left_in ^ self.first_out {
            false => self.right.pop_front(),
            true => self.left.pop_front(),
        };
        self.first_out = false;
        x.expect("Question says all calls to pop will be valid")
    }
    
    fn top(&self) -> i32 {
        *match self.left_in ^ self.first_out {
            true => self.left.front(),
            false => self.right.front(),
        }.expect("Question says all calls to top will be valid")
        
    }
    
    fn empty(&self) -> bool {
        self.left.is_empty() && self.right.is_empty()
    }
}

/**
 * Your MyStack object will be instantiated and called as such:
 * let obj = MyStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: bool = obj.empty();
 */
