//! # Prime Number Check
//! 
//! **Difficulty**: 🟢 Beginner
//! 
//! Check if a number is prime
//!
//! ## Problem Link
//! https://github.com/aarambh-darshan/100-rust-problems/blob/main/problems/012_prime_number_check.md


struct Solution;

impl Solution {
    pub fn is_prime(n: i32) -> bool {
        if n <= 1 {
            return false;
        }
        for i in 2..=((n as f64).sqrt() as i32) {
            if n % i == 0 {
                return false;
            }
        }
        true       
    }
}

/// Solution for Prime Number Check
pub fn solve() {
    // TODO: Implement your solution here
    unimplemented!("Solve Prime Number Check");

    println!("Prime numbers from 1 to 30:");
    let primes: Vec<i32> = (1..=30).filter(|&n| Solution::is_prime(n)).collect();
    println!("{:?}", primes);
    // Output: [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]
    
    // Test specific numbers
    println!("\nTesting specific numbers:");
    let test_cases = [0, 1, 2, 3, 4, 17, 18, 97, 100];
    for n in test_cases {
        println!("{:>3} is prime: {}", n, Solution::is_prime(n));
    }
    
    // Count primes up to 100
    let count = (1..=100).filter(|&n| Solution::is_prime(n)).count();
    println!("\nNumber of primes from 1-100: {}", count);  // 25
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        // TODO: Add test cases
        assert_eq!(Solution::is_prime(2), true);
        assert_eq!(Solution::is_prime(3), true);
        assert_eq!(Solution::is_prime(4), false);
        assert_eq!(Solution::is_prime(5), true);
        assert_eq!(Solution::is_prime(0), false);
        assert_eq!(Solution::is_prime(1), false);
        assert_eq!(Solution::is_prime(-1), false);
        assert_eq!(Solution::is_prime(-2), false);
    }
}
