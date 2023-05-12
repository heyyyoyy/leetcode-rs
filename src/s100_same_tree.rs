// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let mut queue = VecDeque::new();
        queue.push_back((p, q));
        while let Some(pair) = queue.pop_front() {
            match pair {
                (Some(f), Some(s)) => {
                    if f.borrow().val == s.borrow().val {
                        queue.push_back((f.borrow().left.clone(), s.borrow().left.clone()));
                        queue.push_back((f.borrow().right.clone(), s.borrow().right.clone()));
                    }
                }
                (None, Some(_)) | (Some(_), None) => return false,
                _ => {}
            }
        }
        true
    }
}
