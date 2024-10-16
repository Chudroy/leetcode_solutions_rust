use crate::models::Solution;
use std::env; // Import the Solution struct from the models module

mod models;
mod solutions;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Please specify a solution to run (e.g., longest_palindrome)");
        return;
    }

    match args[1].as_str() {
        "longest_palindrome" => {
            let s = String::from("abababa");
            let result = Solution::longest_palindrome(s);
            println!("Longest palindrome is: {}", result);
        }
        "add_two_numbers" => {
            let l1 = Some(Box::new(models::ListNode {
                val: 2,
                next: Some(Box::new(models::ListNode {
                    val: 4,
                    next: Some(Box::new(models::ListNode { val: 9, next: None })),
                })),
            }));
            let l2 = Some(Box::new(models::ListNode {
                val: 5,
                next: Some(Box::new(models::ListNode {
                    val: 6,
                    next: Some(Box::new(models::ListNode { val: 4, next: None })),
                })),
            }));
            let result = Solution::add_two_numbers(l1, l2);
            println!("Sum of the two numbers is: {:?}", result);
        }
        "update_matrix_bfs" => {
            let mat = vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]];
            let result = Solution::update_matrix_bfs(mat);
            println!("Updated matrix (BFS):");
            for row in result {
                println!("{:?}", row);
            }
        }
        "update_matrix_dp" => {
            let mat = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
            let result = Solution::update_matrix_dp(mat);
            println!("Updated matrix (DP):");
            for row in result {
                println!("{:?}", row);
            }
        }
        _ => eprintln!("Solution not found."),
    }
}
