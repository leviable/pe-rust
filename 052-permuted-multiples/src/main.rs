// It can be seen that the number, 125874, and its double, 251748, contain exactly
// the same digits, but in a different order.
//
// Find the smallest positive integer, x, such that 2x, 3x, 4x, 5x, and 6x,
// contain the same digits.

use itertools::Itertools;
use std::collections::HashSet;
use std::time::Instant;

fn same_digits(nums: Vec<u64>) -> bool {
    // Convert all to chars and sort
    let mut hs: HashSet<String> = HashSet::new();

    for num in nums.iter() {
        hs.insert(
            num.to_string()
                .chars()
                .collect::<Vec<_>>()
                .iter()
                .sorted()
                .join(""),
        );
    }

    hs.len() == 1
}

#[test]
fn test_same_digits() {
    assert_eq!(same_digits(vec![125874, 251748]), true);
    assert_eq!(same_digits(vec![125874, 251749]), false);
}

fn pe052() -> u64 {
    for num in 1.. {
        let mut nums: Vec<u64> = vec![];
        for scaler in 1..7 {
            nums.push(num * scaler);
        }
        if same_digits(nums) {
            return num;
        }
    }
    0
}

fn main() {
    let start = Instant::now();
    println!("PE-052: {}", pe052());
    println!("Time elapsed: {}", start.elapsed().as_secs_f64());
}
