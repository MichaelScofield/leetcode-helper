use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use super::def::ListNode;
use super::def::TreeNode;

#[allow(dead_code)]
pub fn vec_to_tree(vec: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    let len = vec.len();
    if len == 0 {
        return None;
    }
    let mut i = 0;
    let root =
        Rc::new(RefCell::new(TreeNode::new(vec[i].unwrap())));
    i += 1;

    let mut queue = VecDeque::new();
    queue.push_back(Rc::clone(&root));

    loop {
        if i >= len {
            break;
        }
        let node = queue.pop_front().unwrap();
        let next_val = vec[i];
        if let Some(left_node_val) = next_val {
            let left_node =
                Rc::new(RefCell::new(TreeNode::new(left_node_val)));
            node.borrow_mut().left = Some(Rc::clone(&left_node));

            queue.push_back(Rc::clone(&left_node));
        }
        i += 1;

        if i >= len {
            break;
        }
        let next_val = vec[i];
        if let Some(right_node_val) = next_val {
            let right_node =
                Rc::new(RefCell::new(TreeNode::new(right_node_val)));
            node.borrow_mut().right = Some(Rc::clone(&right_node));

            queue.push_back(Rc::clone(&right_node));
        }
        i += 1;
    }
    return Some(Rc::clone(&root));
}

/// -1 means "null"
#[allow(dead_code)]
pub fn create_binary_tree(v: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    let len = v.len();
    if len == 0 {
        return None;
    }
    let mut i = 0;
    let root = Rc::new(RefCell::new(TreeNode::new(v[i])));
    i += 1;

    let mut queue = VecDeque::new();
    queue.push_back(Rc::clone(&root));

    loop {
        if i >= len {
            break;
        }
        let node = queue.pop_front().unwrap();
        let left_node_val = v[i];
        if left_node_val >= 0 {
            let left_node =
                Rc::new(RefCell::new(TreeNode::new(left_node_val)));
            node.borrow_mut().left = Some(Rc::clone(&left_node));

            queue.push_back(Rc::clone(&left_node));
        }
        i += 1;

        if i >= len {
            break;
        }
        let right_node_val = v[i];
        if right_node_val >= 0 {
            let right_node =
                Rc::new(RefCell::new(TreeNode::new(right_node_val)));
            node.borrow_mut().right = Some(Rc::clone(&right_node));

            queue.push_back(Rc::clone(&right_node));
        }
        i += 1;
    }
    return Some(Rc::clone(&root));
}

#[allow(dead_code)]
pub fn tree_to_vec(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
    let mut vec = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back(root);

    while let Some(node) = queue.pop_front() {
        if let Some(node) = node {
            let node = node.borrow();
            vec.push(Some(node.val));

            if node.left.is_some() || node.right.is_some() {
                if let Some(ref left_node) = node.left {
                    queue.push_back(Some(left_node.clone()));
                } else {
                    queue.push_back(None);
                }

                if let Some(ref right_node) = node.right {
                    queue.push_back(Some(right_node.clone()));
                } else {
                    queue.push_back(None);
                }
            }
        } else {
            vec.push(None);
        }
    }
    vec
}

#[allow(dead_code)]
pub fn to_string_vec(a: Vec<&str>) -> Vec<String> {
    a.into_iter().map(|s| s.to_string()).collect()
}

#[allow(dead_code)]
pub fn cast(a: Vec<Vec<&str>>) -> Vec<Vec<char>> {
    let mut b = Vec::new();
    for vec_str in a {
        b.push(vec_str.iter().map(|s| s.chars().next().unwrap()).collect());
    }
    b
}

#[allow(dead_code)]
pub fn nums_to_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    for &val in nums.iter().rev() {
        let next = head.take();
        head = Some(Box::new(ListNode { val, next }));
    }
    head
}

#[allow(dead_code)]
pub fn list_to_num(node: Option<Box<ListNode>>) -> i32 {
    let mut n = 0;
    let mut p = 1;
    let mut curr = node;
    while let Some(node) = curr {
        n += node.val * p;
        p *= 10;
        curr = node.next;
    }
    n
}