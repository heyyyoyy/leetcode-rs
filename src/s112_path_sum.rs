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
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if root.is_none() {
            return false;
        }
        let mut queue = VecDeque::new();
        queue.push_back((target_sum, root.unwrap()));
        while let Some((target, node)) = queue.pop_front() {
            let node = node.borrow();
            let new_target = target - node.val;
            if node.left.is_none() && node.right.is_none() && new_target == 0 {
                return true;
            }
            if let Some(l) = node.left.clone() {
                queue.push_back((new_target, l));
            }
            if let Some(r) = node.right.clone() {
                queue.push_back((new_target, r));
            }
        }
        false
    }
}
