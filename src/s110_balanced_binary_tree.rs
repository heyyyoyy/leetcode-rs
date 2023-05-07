// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

type OptionBTree = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    fn deep(root: OptionBTree, mut balanced: bool) -> (i32, bool) {
        if let Some(node) = root {
            if !balanced {
                return (0, balanced);
            }
            let (left_deep, left_balanced) =
                Self::deep(node.as_ref().borrow().left.clone(), balanced);
            balanced = left_balanced;
            let (right_deep, right_balanced) =
                Self::deep(node.as_ref().borrow().right.clone(), balanced);
            balanced = right_balanced;
            if (left_deep - right_deep).abs() > 1 {
                balanced = false;
            }
            (left_deep.max(right_deep) + 1, balanced)
        } else {
            (0, balanced)
        }
    }
    pub fn is_balanced(root: OptionBTree) -> bool {
        Self::deep(root, true).1
    }
}
