// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}
use std::borrow::Borrow;
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;

struct Solution;

impl Solution {
    fn calc_depth(root: Option<Rc<RefCell<TreeNode>>>, depth: i32) -> i32 {
        let left_depth = if let Some(r) = root.clone() {
            Solution::calc_depth(r.as_ref().borrow().left.clone(), depth + 1)
        } else {
            depth
        };
        let right_depth = if let Some(r) = root.clone() {
            Solution::calc_depth(r.as_ref().borrow().right.clone(), depth + 1)
        } else {
            depth
        };
        left_depth.max(right_depth)
    }

    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::calc_depth(root, 1)
    }
}