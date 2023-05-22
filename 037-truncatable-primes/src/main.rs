// The number 3797 has an interesting property. Being prime itself, it is possible to continuously remove digits from left to right, and remain prime at each stage: 3797, 797, 97, and 7. Similarly we can work from right to left: 3797, 379, 37, and 3.

// Find the sum of the only eleven primes that are both truncatable from left to right and right to left.

// NOTE: 2, 3, 5, and 7 are not considered to be truncatable primes.

use primes::{is_prime, PrimeSet, Sieve};
use std::time::Instant;

fn is_right_trunc(mut num: u64) -> bool {
    while num > 9 {
        num /= 10;
        if !is_prime(num) {
            return false;
        }
    }

    true
}

#[test]
fn test_is_right_trunc() {
    assert!(is_right_trunc(3797));
    assert!(!is_right_trunc(19));
}

fn is_left_trunc(mut num: u64) -> bool {
    while num > 9 {
        let num_len = &num.to_string().len() - 1;
        num = num % (10_u64.pow(num_len.try_into().unwrap()));
        if !is_prime(num) {
            return false;
        }
    }

    true
}

fn pe037() -> u64 {
    let mut sieve = Sieve::new();

    let mut prime_count: u64 = 0;
    let mut total: u64 = 0;

    for s in sieve.iter() {
        if is_left_trunc(s) && is_right_trunc(s) {
            if s < 10 {
                continue;
            }
            prime_count += 1;
            total += s;
        }

        if prime_count >= 11 {
            break;
        }
    }

    total
}

#[test]
fn test_is_left_trunc() {
    assert!(is_left_trunc(3797));
    assert!(!is_left_trunc(41));
}

fn main() {
    let start = Instant::now();
    let solution = pe037();
    println!("Solution: {}", solution);
    println!("Time elpased: {}", start.elapsed().as_secs_f64());
}
