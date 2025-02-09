use std::collections::VecDeque;

// Just modelled the question very directly using a VecDeck
impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut queue = VecDeque::from(students);
        let mut num_students = (&queue).len() as i32;
        for s in sandwiches.iter() {
            let mut counter = 0;
            while (Some(s) != queue.front()) {
                if let Some(sad_student) = queue.pop_front() {
                    queue.push_back(sad_student);
                    counter += 1;
                    if counter == num_students {
                        return num_students;
                    }
                }
            }
            queue.pop_front();
            num_students -= 1;
        }
        num_students
    }
}
