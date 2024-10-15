use crate::models::Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let len = s.len(); // Get the length of the string
        if len < 2 {
            return s; // If the string is empty or has one character, it's already a palindrome
        }

        let s_chars: Vec<char> = s.chars().collect(); // Convert the string to a vector of characters

        for i in 0..len {
            // Here we will add logic to expand around each center
            println!("Character at index {}: {}", i, s_chars[i]);
        }

        String::from("placeholder result") // Return a placeholder for now
    }
}
