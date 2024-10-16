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
        "remove_duplicates" => {
            let mut nums = vec![1, 1, 2, 2, 3, 4, 4, 5];
            let result = Solution::remove_duplicates(&mut nums);
            println!("Number of unique elements: {}", result);
            println!("Modified array: {:?}", &nums[..result as usize]);
        }
        _ => eprintln!("Solution not found."),
    }
}
