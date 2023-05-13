use std::cell::RefCell;
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
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];
        if root.is_none() {
            return ans;
        }
        let mut queue = vec![root.unwrap()];
        while let Some(node) = queue.pop() {
            let node = node.borrow();
            ans.push(node.val);
            if let Some(r) = node.right.clone() {
                queue.push(r);
            }
            if let Some(l) = node.left.clone() {
                queue.push(l);
            }
        }
        ans
    }
}
