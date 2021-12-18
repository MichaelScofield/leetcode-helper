use crate::helper::def::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

mod helper;

struct Solution;

impl Solution {
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        assert!(root.is_some());

        fn inorder(
            node: Option<Rc<RefCell<TreeNode>>>,
            pre: &mut Option<Rc<RefCell<TreeNode>>>,
            a: &mut Option<Rc<RefCell<TreeNode>>>,
            b: &mut Option<Rc<RefCell<TreeNode>>>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(node) = node {
                if let Some(left) = node.borrow().left.as_ref() {
                    *pre = inorder(Some(Rc::clone(left)), pre, a, b);
                }

                let val = node.borrow().val;
                if let Some(pre) = pre {
                    let pre_val = pre.borrow().val;
                    if val < pre_val {
                        *b = Some(Rc::clone(&node));
                        if a.is_none() {
                            *a = Some(Rc::clone(&pre));
                        } else {
                            return None;
                        }
                    }
                }

                return if let Some(right) = node.borrow().right.as_ref() {
                    inorder(Some(Rc::clone(right)), &mut Some(Rc::clone(&node)), a, b)
                } else {
                    Some(Rc::clone(&node))
                };
            }
            None
        }

        let mut a = None;
        let mut b = None;

        let mut pre = None;
        inorder(root.clone(), &mut pre, &mut a, &mut b);

        let a = a.unwrap();
        let b = b.unwrap();
        let t = a.borrow().val;
        a.borrow_mut().val = b.borrow().val;
        b.borrow_mut().val = t;
    }
}

fn main() {
    use helper::util::create_binary_tree;
    use helper::util::tree_to_vec;

    let mut root = create_binary_tree(vec![2, 1, -1, -1, 3]);
    Solution::recover_tree(&mut root);
    println!("{:?}", tree_to_vec(root));

    let mut root = create_binary_tree(vec![1, 3, -1, -1, 2]);
    Solution::recover_tree(&mut root);
    println!("{:?}", tree_to_vec(root));

    let mut root = create_binary_tree(vec![3, 1, 4, -1, -1, 2]);
    Solution::recover_tree(&mut root);
    println!("{:?}", tree_to_vec(root));
}
