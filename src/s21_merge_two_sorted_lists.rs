// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

struct Solution;

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (Some(f), None) => return Some(f),
            (None, Some(s)) => return Some(s),
            (None, None) => return None,
            (Some(f), Some(s)) => {
                if f.val <= s.val {
                    return Some(Box::new(ListNode {
                        val: f.val,
                        next: Self::merge_two_lists(f.next, Some(s)),
                    }));
                } else {
                    return Some(Box::new(ListNode {
                        val: s.val,
                        next: Self::merge_two_lists(Some(f), s.next),
                    }));
                }
            }
        }
    }
}
