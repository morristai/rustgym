//! https://leetcode.com/problems/invert-binary-tree/description/

struct Solution;

use std::mem;
use rustgym_util::*;

impl Solution {
    fn invert_tree(root: TreeLink) -> TreeLink {
        // ============ Recursive ============
        // if let Some(node) = &root {
        //     let mut node = node.borrow_mut();
        //     let left = node.left.take();
        //     let right = node.right.take();
        //     node.right = Self::invert_tree(left);
        //     node.left = Self::invert_tree(right);
        // }
        // root

        // ============ Iterative ============
        let mut stack = vec![];
        stack.push(root.clone());

        while !stack.is_empty() {
            if let Some(Some(node)) = stack.pop() {
                let mut borrowed = node.borrow_mut();
                let borrowed = &mut *borrowed;
                // NOTE: You can't pre clone cause the swap will only swap the Rc pointer not the value.
                // let mut left = borrowed.left.clone();
                // let mut right = borrowed.right.clone();
                mem::swap(&mut borrowed.left, &mut borrowed.right);
                stack.push(borrowed.left.clone());
                stack.push(borrowed.right.clone());
            }
        }

        root
    }
}

#[test]
fn test() {
    let input = tree!(
        4,
        tree!(2, tree!(1), tree!(3)),
        tree!(7, tree!(6), tree!(9))
    );
    let output = tree!(
        4,
        tree!(7, tree!(9), tree!(6)),
        tree!(2, tree!(3), tree!(1))
    );
    assert_eq!(Solution::invert_tree(input), output);
}
