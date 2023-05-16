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
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let diameter = 0;
        Self::dfs(root, diameter).1
    }

    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, diameter: i32) -> (i32, i32) {
        if root.is_none() {
            return (0, 0);
        }
        let root = root.unwrap();
        let (left_max_depth, ld) = Self::dfs(root.borrow().left.clone(), diameter);
        let (right_max_depth, rd) = Self::dfs(root.borrow().right.clone(), diameter);
        (
            i32::max(left_max_depth, right_max_depth) + 1,
            i32::max(left_max_depth + right_max_depth, i32::max(ld, rd)),
        )
    }
}
