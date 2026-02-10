//! # Palindrome Number
//! 
//! **Difficulty**: 🟢 Beginner
//! 
//! Check if a number is a palindrome
//!
//! ## Problem Link
//! https://github.com/aarambh-darshan/100-rust-problems/blob/main/problems/003_palindrome_number.md

struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 || (x % 10 == 0 && x != 0) {
            return false;
        }       
        
        let mut reverted_number = 0;
        let mut original_number = x;

        while original_number > reverted_number {
            reverted_number = reverted_number * 10 + original_number % 10;
            original_number /= 10;
        }

        original_number == reverted_number || original_number == reverted_number / 10
    }
}

/// Solution for Palindrome Number
pub fn solve() {
    // TODO: Implement your solution here
    unimplemented!("Solve Palindrome Number");

        // Test Case 1: Palindrome
    println!("121 is palindrome: {}", Solution::is_palindrome(121)); // true
    
    // Test Case 2: Negative number
    println!("-121 is palindrome: {}", Solution::is_palindrome(-121)); // false
    
    // Test Case 3: Not palindrome
    println!("10 is palindrome: {}", Solution::is_palindrome(10)); // false
    
    // Test Case 4: Single digit
    println!("7 is palindrome: {}", Solution::is_palindrome(7)); // true
    
    // Test Case 5: Even length palindrome
    println!("1221 is palindrome: {}", Solution::is_palindrome(1221)); // true
    
    // Test Case 6: Zero
    println!("0 is palindrome: {}", Solution::is_palindrome(0)); // true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        // TODO: Add test cases
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(-121), false);
        assert_eq!(Solution::is_palindrome(10), false);
        assert_eq!(Solution::is_palindrome(12321), true);
    }
}
