impl Solution {
    pub fn is_valid(s: String) -> bool {
        // NOTE: Could be better if when pushing to stack, you push what you're expecting, rather than what you got, but the below is what I did first...
        // NOTE: Question says that s consists of parentheses only '()[]{}'
        if (s.len() % 2 != 0) {
            return false;
        }

        let mut stack = Vec::new();
        for c in s.chars() {
            match c {
                '(' | '{' | '[' => stack.push(c),
                ')' if stack.pop() != Some('(') => return false,
                '}' if stack.pop() != Some('{') => return false,
                ']' if stack.pop() != Some('[') => return false,
                _ => (),
            }

        }
        stack.is_empty()
    }
}
