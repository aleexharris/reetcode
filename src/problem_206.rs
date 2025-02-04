impl Solution {
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // With linked list questions, the playbook seems to be mut pointer to prev, curr & next
        let (mut prev, mut curr) = (None, head);
        while let Some(mut node) = curr {
            let mut next = node.next;
            node.next = prev;
            prev = Some(node);
            curr = next;
        }
        prev
    }
}
