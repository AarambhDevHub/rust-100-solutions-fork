//! # FizzBuzz
//! 
//! **Difficulty**: 🟢 Beginner
//! 
//! Classic FizzBuzz problem
//!
//! ## Problem Link
//! https://github.com/aarambh-darshan/100-rust-problems/blob/main/problems/004_fizzbuzz.md

struct Solution;

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut result = Vec::new();

        for i in 1..=n {
            // if i % 3 == 0 && i % 5 == 0 {
            //     result.push("FizzBuzz".to_string());
            // } else if i % 3 == 0 {
            //     result.push("Fizz".to_string());
            // } else if i % 5 == 0 {
            //     result.push("Buzz".to_string());
            // } else {
            //     result.push(i.to_string());
            // }

            let output = match (i % 3, i % 5) {
                (0, 0) => "FizzBuzz".to_string(),
                (0, _) => "Fizz".to_string(),
                (_, 0) => "Buzz".to_string(),
                _ => i.to_string(),
            };
            result.push(output);
        }

        result
    }
}

/// Solution for FizzBuzz
pub fn solve() {
    // TODO: Implement your solution here
    unimplemented!("Solve FizzBuzz");

    // Test Case 1: n = 3
    let result1 = Solution::fizz_buzz(3);
    println!("n=3: {:?}", result1);
    // Output: ["1", "2", "Fizz"]
    
    // Test Case 2: n = 5
    let result2 = Solution::fizz_buzz(5);
    println!("n=5: {:?}", result2);
    // Output: ["1", "2", "Fizz", "4", "Buzz"]
    
    // Test Case 3: n = 15
    let result3 = Solution::fizz_buzz(15);
    println!("n=15: {:?}", result3);
    // Output: ["1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", 
    //          "11", "Fizz", "13", "14", "FizzBuzz"]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        // TODO: Add test cases

        let expected = vec![
            "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz",
            "11", "Fizz", "13", "14", "FizzBuzz"
        ].iter().map(|&s| s.to_string()).collect::<Vec<String>>();
        assert_eq!(Solution::fizz_buzz(15), expected);

        let expected = vec![
            "1", "2", "Fizz", "4", "Buzz"
        ].iter().map(|&s| s.to_string()).collect::<Vec<String>>();
        assert_eq!(Solution::fizz_buzz(5), expected);

        let expected = vec![
            "1", "2", "Fizz"
        ].iter().map(|&s| s.to_string()).collect::<Vec<String>>();
        assert_eq!(Solution::fizz_buzz(3), expected);

        let expected = vec!["1"].iter().map(|&s| s.to_string()).collect::<Vec<String>>();
        assert_eq!(Solution::fizz_buzz(1), expected);
    }
}
