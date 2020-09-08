use std::cell::RefCell;
use std::rc::Rc;

#[macro_export]
macro_rules! vecvec {
    () => (
        Vec::with_capacity(0)
    );
    ($([ $($x:expr),* ]),*) => ({
        let mut _vv = Vec::new();
        $(
            let mut _v = Vec::new();
            $(
                _v.push($x);
            )*
            _vv.push(_v);
        )*
        _vv
    })
}

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

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[allow(dead_code)]
impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}