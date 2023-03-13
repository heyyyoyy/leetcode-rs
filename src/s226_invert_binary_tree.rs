// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    // swap nodes with std::mem::swap
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(r) = root.clone() {
            let TreeNode { left, right, .. } = &mut *r.borrow_mut();
            Solution::invert_tree(left.clone());
            Solution::invert_tree(right.clone());
            std::mem::swap(left, right);
        }
        root
    }

    // swap with create new root
    pub fn invert_tree_new_root(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(r) = root {
            let r = r.borrow();
            let mut new_root = TreeNode::new(r.val);
            new_root.left = Self::invert_tree_new_root(r.right.clone());
            new_root.right = Self::invert_tree_new_root(r.left.clone());
            Some(Rc::new(RefCell::new(new_root)))
        } else {
            None
        }
    }
}
