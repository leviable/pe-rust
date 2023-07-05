// There are exactly ten ways of selecting three from five, 12345:
//
//       123, 124, 125, 134, 135, 145, 234, 235, 245, and 345
//
// In combinatorics, we use the notation, (5|3) = 10 .
//
// In general, (n | r) =     n!
//                        --------
//                        r!(n-r)!
//
// , where r <= n , and n! = n * (n-1) * (n-2) ... * 3 * 2 * 1
//
// It is not until n = 23 that a value exceeds one-million: (23 | 10) = 1144066
//
// How many, not necessarily distinct, values of (n|r) for 1 <= n <= 100,
// are greater than one-million?

use primes::{PrimeSet, Sieve};
use std::time::Instant;

fn factorial(num: u64) -> Vec<u64> {
    let mut v = vec![];
    for f in 2..=num {
        v.push(f);
    }
    v.reverse();
    v
}

#[test]
fn test_factorial() {
    assert_eq!(factorial(10), vec![10, 9, 8, 7, 6, 5, 4, 3, 2]);
    assert_eq!(factorial(5), vec![5, 4, 3, 2]);
}

fn common_denoms(num: u64) -> Vec<u64> {
    let mut denoms = vec![];
    let mut val = num;
    for prime in Sieve::new().iter() {
        if prime > val {
            break;
        }
        loop {
            if val % prime == 0 {
                denoms.push(prime);
                val /= prime;
            } else {
                break;
            }
        }
    }

    denoms
}

#[test]
fn test_common_denoms() {
    assert_eq!(common_denoms(10), vec![2, 5]);
    assert_eq!(common_denoms(17), vec![17]);
    assert_eq!(common_denoms(8), vec![2, 2, 2]);
}

fn combinate(n: u64, r: u64) -> u64 {
    let top = factorial(n);
    let bottom = factorial(r).extend(factorial(n - r));

    0
}

#[test]
fn test_combinate() {
    assert_eq!(combinate(5, 3), 10);
    assert_eq!(combinate(23, 10), 1144066);
}

fn main() {
    let start = Instant::now();

    println!("Time elapsed: {}", start.elapsed().as_secs_f64());
}
