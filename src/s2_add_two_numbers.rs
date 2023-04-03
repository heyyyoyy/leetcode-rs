use num::Integer;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

struct Solution;

impl Solution {
    pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut root = ListNode::new(0);
        let mut cur = &mut root;
        let mut carry = 0;
        while l1.is_some() || l2.is_some() || carry != 0 {
            let mut sum = 0;
            if let Some(l1_node) = l1 {
                sum += l1_node.val;
                l1 = l1_node.next;
            }
            if let Some(l2_node) = l2 {
                sum += l2_node.val;
                l2 = l2_node.next;
            }
            sum += carry;
            let (quotient, rem) = sum.div_rem(&10);
            carry = quotient;
            let new_node = ListNode::new(rem);
            cur.next = Some(Box::new(new_node));
            cur = cur.next.as_mut().unwrap();

        }
        root.next
    }

    fn to_vec(list_node: ListNode) -> Vec<i32> {
        let mut vec = vec![list_node.val];
        let mut cur = list_node.next;
        while let Some(node) = cur {
            vec.push(node.val);
            cur = node.next;
        }
        vec
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_add_two_numbers() {
        let l1 = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode {
                     val: 3, next: None 
                    }))
            }))
        }));
        let l2 = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode {
                     val: 4, next: None 
                    }))
            }))
        }));
        let res = Solution::add_two_numbers(l1, l2);
        assert_eq!(vec![7,0,8], Solution::to_vec(*res.unwrap()))
    }
}