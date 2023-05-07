// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

use std::cell::RefCell;
use std::rc::Rc;

type Tree = Option<Rc<RefCell<TreeNode>>>;
struct Solution;

impl Solution {
    fn get_path(root: Tree, path: &mut Vec<Tree>, target: i32) -> bool {
        if let Some(node) = root.clone() {
            path.push(Some(node.clone()));
            let n = node.borrow();
            if target == n.val {
                return true;
            }
            if (n.left.is_some() && Self::get_path(n.left.clone(), path, target))
                || (n.right.is_some() && Self::get_path(n.right.clone(), path, target))
            {
                return true;
            }
            path.pop();
        }
        false
    }

    pub fn lowest_common_ancestor(root: Tree, p: Tree, q: Tree) -> Tree {
        let mut path_f = Vec::new();
        let mut path_s = Vec::new();
        let mut answer = None;
        Self::get_path(root.clone(), &mut path_f, p.unwrap().borrow().val);
        Self::get_path(root.clone(), &mut path_s, q.unwrap().borrow().val);
        for idx in 0..path_f.len().max(path_s.len()) {
            match (path_f.get(idx), path_s.get(idx)) {
                (Some(x), Some(y)) => {
                    let rc_f = x.clone().unwrap();
                    let val_f = rc_f.borrow().val;
                    let rc_s = y.clone().unwrap();
                    let val_s = rc_s.borrow().val;
                    if val_f == val_s {
                        answer = Some(rc_f);
                    }
                }
                _ => break,
            }
        }
        answer
    }
}
