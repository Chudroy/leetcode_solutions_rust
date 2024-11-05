use crate::models::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub fn print_tree(root: Option<Rc<RefCell<TreeNode>>>) {
    if root.is_none() {
        println!("Empty tree");
        return;
    }

    let mut queue = VecDeque::new();
    queue.push_back(root);

    while !queue.is_empty() {
        let level_size = queue.len();
        let mut level_nodes = Vec::new();

        for _ in 0..level_size {
            if let Some(Some(node_rc)) = queue.pop_front() {
                let node = node_rc.borrow();
                level_nodes.push(node.val.to_string());

                // Enqueue left and right children
                if node.left.is_some() {
                    queue.push_back(node.left.clone());
                } else {
                    queue.push_back(None);
                }

                if node.right.is_some() {
                    queue.push_back(node.right.clone());
                } else {
                    queue.push_back(None);
                }
            } else {
                level_nodes.push("null".to_string());
            }
        }

        // Check if the entire level is empty (all "null")
        if level_nodes.iter().all(|x| x == "null") {
            break;
        }

        println!("{}", level_nodes.join(" "));
    }
}

// Function to print the tree in a pretty format
pub fn print_tree_pretty(root: Option<Rc<RefCell<TreeNode>>>, prefix: String, is_left: bool) {
    if let Some(node_rc) = root {
        let node = node_rc.borrow();
        println!(
            "{}{}-- {}",
            prefix,
            if is_left { "├" } else { "└" },
            node.val
        );

        let child_prefix = format!("{}{}", prefix, if is_left { "│   " } else { "    " });

        // Right child
        print_tree_pretty(node.right.clone(), child_prefix.clone(), true);

        // Left child
        print_tree_pretty(node.left.clone(), child_prefix, false);
    }
}
