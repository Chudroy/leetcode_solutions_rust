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
            let s = String::from("babad");
            let result = Solution::longest_palindrome(s);
            println!("Longest palindrome is: {}", result);
        }
        _ => eprintln!("Solution not found."),
    }
}
