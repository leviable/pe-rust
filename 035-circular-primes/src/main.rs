// The number, 197, is called a circular prime because all rotations of the digits: 197, 971, and 719, are themselves prime.

// There are thirteen such primes below 100: 2, 3, 5, 7, 11, 13, 17, 31, 37, 71, 73, 79, and 97.

// How many circular primes are there below one million?

use primes::{is_prime, PrimeSet, Sieve};
use std::time::Instant;

fn is_circular(num: usize) -> bool {
    let num_str = &num.to_string();
    let mut num_chars: Vec<_> = num_str.chars().collect();

    for _ in 0..num_chars.len() {
        num_chars.rotate_left(1);

        let test_val: String = num_chars.iter().collect();
        if !is_prime(test_val.parse().unwrap()) {
            return false;
        }
    }

    true
}

#[test]
fn test_is_circular() {
    assert!(
        !is_circular(19),
        "Expected 19 to be not circular, but got true"
    );
    assert!(
        is_circular(197),
        "Expected 197 to be circular, but got false"
    );
    assert!(
        !is_circular(200),
        "Expected 200 to be not circular, but got true"
    );
}

fn primes_below(limit: usize) -> usize {
    let mut prime_count = 0;
    let mut pset = Sieve::new();
    for n in pset.iter() {
        if n > limit as u64 {
            break;
        }

        if is_circular(n as usize) {
            prime_count += 1;
        } else {
        }
    }

    prime_count
}

#[test]
fn test_primes_below() {
    assert_eq!(primes_below(100), 13);
}

fn main() {
    let start = Instant::now();
    let solution = primes_below(1_000_000);
    println!("Problem 35 solution: {solution}");
    println!("Time elapsed: {}", start.elapsed().as_secs_f64());
}
