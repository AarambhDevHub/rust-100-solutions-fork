//! # Leap Year
//! 
//! **Difficulty**: 🟢 Beginner
//! 
//! Determine if a year is a leap year
//!
//! ## Problem Link
//! https://github.com/aarambh-darshan/100-rust-problems/blob/main/problems/013_leap_year.md

struct Solution;

impl Solution {
    pub fn is_leap_year(year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }
}

/// Solution for Leap Year
pub fn solve() {
    // TODO: Implement your solution here
    unimplemented!("Solve Leap Year");

    // Test various years
    let years = [1900, 2000, 2004, 2019, 2020, 2021, 2024, 2100];
    
    println!("{:>6} | {:>10}", "Year", "Leap Year?");
    println!("{}", "-".repeat(20));
    
    for year in years {
        println!("{:>6} | {:>10}", year, Solution::is_leap_year(year));
    }
    
    // Famous leap year edge cases
    println!("\n--- Notable Examples ---");
    println!("1900: {} (divisible by 100, not 400)", Solution::is_leap_year(1900));
    println!("2000: {} (divisible by 400)", Solution::is_leap_year(2000));
    println!("2024: {} (divisible by 4, not 100)", Solution::is_leap_year(2024));
    
    // Count leap years in a range
    let leap_years: Vec<i32> = (2000..=2030)
        .filter(|&y| Solution::is_leap_year(y))
        .collect();
    println!("\nLeap years 2000-2030: {:?}", leap_years);

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        // TODO: Add test cases
        assert_eq!(Solution::is_leap_year(2000), true);
        assert_eq!(Solution::is_leap_year(1900), false);
        assert_eq!(Solution::is_leap_year(2004), true);
        assert_eq!(Solution::is_leap_year(2001), false);
        assert_eq!(Solution::is_leap_year(1600), true);
        assert_eq!(Solution::is_leap_year(1700), false);
        assert_eq!(Solution::is_leap_year(2024), true);
    }
}
