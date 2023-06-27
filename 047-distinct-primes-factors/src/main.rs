// The first two consecutive numbers to have two distinct prime factors are:
//
// 14 = 2 x 7
// 15 = 3 x 5
//
//
// The first three consecutive numbers to have three distinct prime factors are:
//
// 644 = 2**2 x 7 x 23
// 645 = 3 x 5 x 43
// 646 = 2 x 17 x 19
//
//
// Find the first four consecutive integers to have four distinct prime factors
// each. What is the first of these numbers?

use primes::{PrimeSet, Sieve};
use std::collections::HashSet;
use std::iter::FromIterator;
use std::time::Instant;

// TODO: Could memoize here to avoid recalculating prime_factors over and over
fn get_prime_factors(mut num: u64) -> Vec<u64> {
    let mut prime_factors: Vec<u64> = vec![];
    for p in Sieve::new().iter() {
        if p > num {
            break;
        }

        while num % p == 0 {
            prime_factors.push(p);
            num /= p;
        }
    }

    prime_factors
}

#[test]
fn test_get_prime_factors() {
    assert_eq!(get_prime_factors(14), vec![2, 7]);
    assert_eq!(get_prime_factors(15), vec![3, 5]);
    assert_eq!(get_prime_factors(644), vec![2, 2, 7, 23]);
    assert_eq!(get_prime_factors(645), vec![3, 5, 43]);
    assert_eq!(get_prime_factors(646), vec![2, 17, 19]);
}

fn find_consecutive_numbers(con_count: usize) -> Vec<u64> {
    let mut found: Vec<u64> = vec![];
    for num in 1.. {
        if num % 1000 == 0 {
            println!("Using: {num}");
        }
        let factors = get_prime_factors(num);
        let h: HashSet<u64> = HashSet::from_iter(factors.iter().cloned());
        if h.len() == con_count {
            found.push(num);
        } else {
            found = vec![];
        }

        if found.len() == con_count {
            break;
        }
    }
    found
}

#[test]
fn test_find_consecutive() {
    assert_eq!(find_consecutive_numbers(2), vec![14, 15]);
    assert_eq!(find_consecutive_numbers(3), vec![644, 645, 646]);
}

fn main() {
    let start = Instant::now();
    let solution = find_consecutive_numbers(4);
    println!("Solution: {:?}", solution);
    println!("Time elapsed: {}", start.elapsed().as_secs_f64());
}
