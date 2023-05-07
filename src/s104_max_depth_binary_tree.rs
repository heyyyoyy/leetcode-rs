pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        root.iter()
            .map(|node| {
                Solution::max_depth(node.as_ref().borrow().left.clone())
                    .max(Solution::max_depth(node.as_ref().borrow().right.clone()))
                    + 1
            })
            .sum()
    }
}
