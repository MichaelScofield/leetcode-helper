use crate::helper::def::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

mod helper;

struct Solution;

impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn traversal(node: Option<Rc<RefCell<TreeNode>>>, path: &mut Vec<i32>, sum: &mut i32) {
            if let Some(node) = node {
                path.push(node.borrow().val);

                let mut is_leaf = true;
                if let Some(left) = node.borrow().left.clone() {
                    is_leaf = false;
                    traversal(Some(left), path, sum);
                }
                if let Some(right) = node.borrow().right.clone() {
                    is_leaf = false;
                    traversal(Some(right), path, sum);
                }
                if is_leaf {
                    *sum += path
                        .iter()
                        .rev()
                        .enumerate()
                        .fold(0, |acc, (i, &n)| acc + n * 10i32.pow(i as u32));
                }

                path.pop();
            }
        }
        let mut path = vec![];
        let mut sum = 0;
        traversal(root, &mut path, &mut sum);
        sum
    }
}

fn main() {
    use helper::util::create_binary_tree;
    assert_eq!(25, Solution::sum_numbers(create_binary_tree(vec![1, 2, 3])));
    assert_eq!(
        1026,
        Solution::sum_numbers(create_binary_tree(vec![4, 9, 0, 5, 1]))
    );
}
