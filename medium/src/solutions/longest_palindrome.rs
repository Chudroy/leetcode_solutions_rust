use crate::models::Solution;


/**
 * Given a string s, return the longest palindromic substring in s.
 */
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let len = s.len();
        if len < 2 {
            return s; // A single character or empty string is already a palindrome
        }

        let s_chars: Vec<char> = s.chars().collect(); // Convert string to a vector of characters
        let mut start = 0; // Track the starting index of the longest palindrome
        let mut end = 0; // Track the ending index of the longest palindrome

        for i in 0..len {
            // Odd-length palindrome expansion (centered at one character)
            let (left1, right1) = Self::expand_around_center(&s_chars, i, i);
            Self::update_longest_palindrome(&mut start, &mut end, left1, right1);

            // Even-length palindrome expansion (centered between two characters)
            if i + 1 < len {
                let (left2, right2) = Self::expand_around_center(&s_chars, i, i + 1);
                Self::update_longest_palindrome(&mut start, &mut end, left2, right2);
            }
        }

        s_chars[start..=end].iter().collect() // Return the longest palindrome substring
    }

    // Expands around a given center and returns the start and end indices of the palindrome
    fn expand_around_center(s_chars: &Vec<char>, left: usize, right: usize) -> (usize, usize) {
        let mut l = left as isize; // Use isize to handle bounds safely
        let mut r = right as isize;

        while l >= 0 && (r as usize) < s_chars.len() && s_chars[l as usize] == s_chars[r as usize] {
            l -= 1;
            r += 1;
        }

        // Return the start and end indices of the palindrome
        ((l + 1) as usize, (r - 1) as usize)
    }

    // Updates the longest palindrome found if the current one is longer
    fn update_longest_palindrome(
        start: &mut usize,
        end: &mut usize,
        new_start: usize,
        new_end: usize,
    ) {
        if new_end >= new_start && (new_end - new_start) > (*end - *start) {
            *start = new_start;
            *end = new_end;
        }
    }
}
