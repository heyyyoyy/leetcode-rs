struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current = head.as_mut();
        while let Some(cur_head) = current {
            while let Some(next) = cur_head.next.take() {
                if cur_head.val == next.val {
                    cur_head.next = next.next;
                } else {
                    cur_head.next = Some(next);
                    break;
                }
            }
            current = cur_head.next.as_mut();
        }
        head
    }
}
