//! # Count Digits
//! 
//! **Difficulty**: 🟢 Beginner
//! 
//! Count the number of digits in a number
//!
//! ## Problem Link
//! https://github.com/aarambh-darshan/100-rust-problems/blob/main/problems/007_count_digits.md

struct Solution;

impl Solution {
    pub fn count_digits(n: i32) -> i32 {
      if n == 0 {
          return 1;
      }

      let mut count = 0;
      let mut num = n.abs();  // Handle negative numbers
      while num > 0 {
          count += 1;
          num /= 10;
      }       
      count
    }

    pub fn count_digits_string(n: i32) -> i32 {
        n.abs().to_string().len() as i32
    }

    pub fn count_digits_log(n: i32) -> i32 {
        if n == 0 {
            return 1;
        }
        (n.abs() as f64).log10().floor() as i32 + 1
    }
}

/// Solution for Count Digits
pub fn solve() {
    // TODO: Implement your solution here
    unimplemented!("Solve Count Digits");

       // Test various numbers
    let test_cases = [0, 1, 9, 10, 99, 100, 12345, -12345, 1000000];
    
    println!("{:>10} | {:>6} | {:>6} | {:>6}", "Number", "Math", "String", "Log");
    println!("{}", "-".repeat(40));
    
    for n in test_cases {
        println!("{:>10} | {:>6} | {:>6} | {:>6}", 
            n, 
            Solution::count_digits(n),
            Solution::count_digits_string(n),
            Solution::count_digits_log(n)
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        // TODO: Add test cases
        println!("Count of digits in 12345: {:?}", Solution::count_digits(12345));
        assert_eq!(Solution::count_digits(0), 1);
        assert_eq!(Solution::count_digits(7), 1);
        assert_eq!(Solution::count_digits(123), 3);
        assert_eq!(Solution::count_digits(-4567), 4); // Test negative number
    }

    #[test]
    fn test_string_method() {
        assert_eq!(Solution::count_digits_string(0), 1);
        assert_eq!(Solution::count_digits_string(7), 1);
        assert_eq!(Solution::count_digits_string(123), 3);
        assert_eq!(Solution::count_digits_string(-4567), 4);
    }

    #[test]
    fn test_log_method() {
        assert_eq!(Solution::count_digits_log(0), 1);
        assert_eq!(Solution::count_digits_log(7), 1);
        assert_eq!(Solution::count_digits_log(123), 3);
        assert_eq!(Solution::count_digits_log(-4567), 4);
    }
}