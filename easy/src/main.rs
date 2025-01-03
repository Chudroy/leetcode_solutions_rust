mod models;
mod solutions;

use models::Solution;
use utils::binary_tree;
use utils::models::{ListNode, TreeNode};

use std::cell::RefCell;
use std::env;
use std::rc::Rc;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Please specify a solution to run (e.g., longest_palindrome)");
        return;
    }

    match args[1].as_str() {
        "remove_duplicates" => {
            let mut nums = vec![1, 1, 2, 2, 3, 4, 4, 5];
            let result = Solution::remove_duplicates(&mut nums);
            println!("Number of unique elements: {}", result);
            println!("Modified array: {:?}", &nums[..result as usize]);
        }
        "valid_parentheses" => {
            let input = String::from("()[]{}");
            let result = Solution::is_valid(input.clone());
            println!("Input: \"{}\"", input);
            println!("Is valid: {}", result);
        }
        "merge_two_lists" => {
            // Create sample linked lists
            let list1 = Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode::new(4))),
                })),
            }));

            let list2 = Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode::new(4))),
                })),
            }));

            let result = Solution::merge_two_lists(list1, list2);

            // Helper function to convert linked list to vector
            fn list_to_vec(list: Option<Box<ListNode>>) -> Vec<i32> {
                let mut vec = Vec::new();
                let mut current = list;
                while let Some(node) = current {
                    vec.push(node.val);
                    current = node.next;
                }
                vec
            }

            let merged_list = list_to_vec(result);
            println!("Merged list: {:?}", merged_list);
        }
        "max_profit" => {
            let prices = vec![7, 1, 5, 3, 6, 4];
            let result = Solution::max_profit(prices);
            println!("Max profit: {}", result);
        }
        "invert_tree" => {
            // Create a sample binary tree
            let tree = Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
                }))),
            })));

            binary_tree::print_tree::print_tree_pretty(tree.clone(), String::new(), false);

            let result = Solution::invert_tree(tree);

            println!("\nInverted tree (Pretty):");
            binary_tree::print_tree::print_tree_pretty(result, String::new(), false);
        }
        _ => eprintln!("Solution not found."),
    }
}
