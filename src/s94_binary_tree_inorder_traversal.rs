struct Solution;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];
        if root.is_none() {
            return ans;
        }
        let mut stack = vec![];
        let mut cur_node = root;
        while cur_node.is_some() || !stack.is_empty() {
            while let Some(node) = cur_node.clone() {
                stack.push(node.clone());
                cur_node = node.borrow().left.clone();
            }
            cur_node = stack.pop();
            let c = cur_node.unwrap();
            ans.push(c.borrow().val);
            cur_node = c.borrow().right.clone();
        }
        ans
    }
}
