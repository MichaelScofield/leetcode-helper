use std::cell::RefCell;
use std::rc::Rc;

use helper::def::TreeNode;

mod helper;

struct Solution;

impl Solution {
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(root) = root {
            let root = root.borrow();

            let mut rob_root = root.val;
            if let Some(left) = &root.left {
                let left = left.borrow();
                rob_root += Solution::rob(left.left.clone()) + Solution::rob(left.right.clone());
            }
            if let Some(right) = &root.right {
                let right = right.borrow();
                rob_root += Solution::rob(right.left.clone()) + Solution::rob(right.right.clone());
            }

            let rob_left = Solution::rob(root.left.clone());
            let rob_right = Solution::rob(root.right.clone());

            std::cmp::max(rob_root, rob_left + rob_right)
        } else {
            0
        }
    }
}

fn main() {
    let root = helper::util::create_binary_tree(vec![2, 1, 3, -1, 4]);
    assert_eq!(7, Solution::rob(root));
    let root = helper::util::create_binary_tree(vec![3, 2, 3, -1, 3, -1, 1]);
    assert_eq!(7, Solution::rob(root));
    let root = helper::util::create_binary_tree(vec![3, 4, 5, 1, 3, -1, 1]);
    assert_eq!(9, Solution::rob(root));
}
