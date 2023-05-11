use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

struct Solution;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut queue = VecDeque::new();
        queue.push_front((1, root));
        let mut depth = 0;
        while let Some((d, Some(node))) = queue.pop_front() {
            let node = node.borrow();
            depth = d;
            match (node.left.clone(), node.right.clone()) {
                (None, None) => break,
                (None, Some(r)) => queue.push_back((d + 1, Some(r))),
                (Some(l), None) => queue.push_back((d + 1, Some(l))),
                (Some(l), Some(r)) => {
                    queue.push_back((d + 1, Some(l)));
                    queue.push_back((d + 1, Some(r)));
                }
            }
        }
        depth
    }
}
