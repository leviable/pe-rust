// We shall say that an n-digit number is pandigital if it makes use of all the
// digits 1 to n exactly once. For example, 2143 is a 4-digit pandigital and is
// also prime.

// What is the largest n-digit pandigital prime that exists?

use primes::{PrimeSet, Sieve};
use std::time::Instant;

const PANDIGITAL: &str = "123456789";

fn is_pandigital(num: u64) -> bool {
    let mut num_str: Vec<String> = num.to_string().chars().map(|x| x.to_string()).collect();
    num_str.sort();
    num_str.join("") == PANDIGITAL[..num_str.len()]
}

#[test]
fn test_is_pandigital() {
    assert!(is_pandigital(2143));
    assert!(!is_pandigital(2142));
}

fn pe041() -> u64 {
    let mut pset = Sieve::new();
    let mut largest = 0;
    for p in pset.iter() {
        // Largest prime is 7652413, not sure why
        if p > 7_652_413 {
            break;
        }
        if is_pandigital(p) && p > largest {
            largest = p;
        }
    }
    largest
}

fn main() {
    let start = Instant::now();
    let solution = pe041();
    println!("Solution is: {solution}");
    println!("Time Elapsed: {}", start.elapsed().as_secs_f64());
}
