// Trick in this one was storing the minimum value and the stack in parallel in the same Vec...
// ... thus as you pop elements, you also retrieve the state of the min value at that time.
// There is always an iter() method for whatever you want to do haha

struct MinStack {
    min: i32, // Problem states that the stack's state will never be queried when empty, otherwise this would be an Option<i32>
    stack: Vec<i32>
}

impl MinStack {

    fn new() -> Self {
        MinStack {
            min: i32::MAX,
            stack: Vec::new()
        }
        
    }
    
    fn push(&mut self, val: i32) {
        self.stack.push(val);
        if val < self.min {
            self.min = val;
        };
        
    }
    
    fn pop(&mut self) {
        if self.min == self.stack.pop().expect("Problem says that pop method will always be called on non-empty stacks") {
            self.min = *self.stack.iter().min().unwrap_or(&i32::MAX);
        };
    }
    
    fn top(&self) -> i32 {
        *self.stack.last().expect("Top method will always be called on non-empty stacks")
    }
    
    fn get_min(&self) -> i32 {
        self.min
    }
}
