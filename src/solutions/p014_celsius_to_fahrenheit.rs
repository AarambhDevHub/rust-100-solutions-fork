//! # Celsius to Fahrenheit
//! 
//! **Difficulty**: 🟢 Beginner
//! 
//! Convert temperature
//!
//! ## Problem Link
//! https://github.com/aarambh-darshan/100-rust-problems/blob/main/problems/014_celsius_to_fahrenheit.md

struct Solution;

impl Solution {
    pub fn celsius_to_fahrenheit(celsius: f64) -> f64 {
        Solution::round_2((celsius * 9.0 / 5.0) + 32.0)
    }
    fn round_2(val: f64) -> f64 {
        format!("{:.2}", val).parse().unwrap()
    }

    fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
        Solution::round_2((fahrenheit - 32.0) * 5.0 / 9.0)
    }


}

/// Solution for Celsius to Fahrenheit
pub fn solve() {
    // TODO: Implement your solution here
    unimplemented!("Solve Celsius to Fahrenheit");

        // Common temperature conversions
    println!("Temperature Conversion Table");
    println!("{:>10} °C | {:>10} °F", "Celsius", "Fahrenheit");
    println!("{}", "-".repeat(26));
    
    let celsius_values = [-40.0, -20.0, 0.0, 10.0, 20.0, 25.0, 30.0, 37.0, 100.0];
    
    for c in celsius_values {
        let f = Solution::celsius_to_fahrenheit(c);
        println!("{:>10.1} °C | {:>10.1} °F", c, f);
    }
    
    // Verify the reverse conversion
    println!("\n--- Verification ---");
    let original = 25.0;
    let converted = Solution::celsius_to_fahrenheit(original);
    let back = Solution::fahrenheit_to_celsius(converted);
    println!("{} °C → {} °F → {} °C", original, converted, back);
    
    // Famous temperatures
    println!("\n--- Notable Temperatures ---");
    println!("Water freezes: {} °C = {} °F", 0.0, Solution::celsius_to_fahrenheit(0.0));
    println!("Water boils: {} °C = {} °F", 100.0, Solution::celsius_to_fahrenheit(100.0));
    println!("Body temp: {} °C = {} °F", 37.0, Solution::celsius_to_fahrenheit(37.0));
    println!("Same value: {} °C = {} °F", -40.0, Solution::celsius_to_fahrenheit(-40.0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        // TODO: Add test cases

        assert_eq!(Solution::celsius_to_fahrenheit(0.0), 32.0);
        assert_eq!(Solution::celsius_to_fahrenheit(100.0), 212.0);
        assert_eq!(Solution::celsius_to_fahrenheit(-40.0), -40.0);
        assert_eq!(Solution::celsius_to_fahrenheit(37.0), 98.6);
        assert_eq!(Solution::celsius_to_fahrenheit(20.0), 68.0);
        assert_eq!(Solution::celsius_to_fahrenheit(-273.15), -459.67);
    }

    #[test]
    fn test_fahrenheit_to_celsius() {
        assert_eq!(Solution::fahrenheit_to_celsius(32.0), 0.0);
        assert_eq!(Solution::fahrenheit_to_celsius(212.0), 100.0);
        assert_eq!(Solution::fahrenheit_to_celsius(-40.0), -40.0);
        assert_eq!(Solution::fahrenheit_to_celsius(98.6), 37.0);
        assert_eq!(Solution::fahrenheit_to_celsius(68.0), 20.0);
        assert_eq!(Solution::fahrenheit_to_celsius(-459.67), -273.15);
    }
}
