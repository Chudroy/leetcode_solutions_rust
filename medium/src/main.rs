use crate::models::Solution;
use std::env; // Import the Solution struct from the models module
pub mod models;
pub mod solutions;

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
        },
        "add_two_numbers" =>{
            let l1 = Some(Box::new(models::ListNode {
                val: 9,
                next: Some(Box::new(models::ListNode {
                    val: 9,
                    next: Some(Box::new(models::ListNode {
                        val: 9,
                        next: None,
                    })),
                })),
            }));
            let l2 = Some(Box::new(models::ListNode {
                val: 5,
                next: Some(Box::new(models::ListNode {
                    val: 6,
                    next: Some(Box::new(models::ListNode {
                        val: 4,
                        next: None,
                    })),
                })),
            }));
            let result = Solution::add_two_numbers(l1, l2);
            println!("Sum of the two numbers is: {:?}", result);
        }
        _ => eprintln!("Solution not found."),
    }
}
