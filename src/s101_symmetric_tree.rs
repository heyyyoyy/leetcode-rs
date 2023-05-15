struct Solution;

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
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut queue = VecDeque::new();
        let root = root.unwrap();
        queue.push_back((root.borrow().left.clone(), root.borrow().right.clone()));
        while let Some((left, right)) = queue.pop_front() {
            match (left, right) {
                (None, None) => continue,
                (None, Some(_)) | (Some(_), None) => return false,
                (Some(l), Some(ref r)) => {
                    if l.borrow().val != r.borrow().val {
                        return false;
                    }
                    queue.push_back((l.borrow().left.clone(), r.borrow().right.clone()));
                    queue.push_back((l.borrow().right.clone(), r.borrow().left.clone()));
                }
            }
        }
        true
    }
}
