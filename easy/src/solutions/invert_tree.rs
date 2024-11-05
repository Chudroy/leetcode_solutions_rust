use utils::models::TreeNode;

use crate::models::Solution;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node_rc) = root {
            {
                let mut node = node_rc.borrow_mut();

                // Recursively invert the left and right subtrees
                let left_inverted = Self::invert_tree(node.left.take());
                let right_inverted = Self::invert_tree(node.right.take());

                // Swap the inverted subtrees
                node.left = right_inverted;
                node.right = left_inverted;
            }

            Some(node_rc)
        } else {
            None
        }
    }
}
