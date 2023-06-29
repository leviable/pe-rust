// The prime 41, can be written as the sum of six consecutive primes:
//
//  41 = 2 + 3 + 4 + 5 + 7 + 11 + 13
//
// This is the longest sum of consecutive primes that adds to a prime below one-hundred.
//
// The longest sum of consecutive primes below one-thousand that adds to a prime, contains
// 21 terms, and is equal to 953.
//
// Which prime, below one-million, can be written as the sum of the most consecutive primes?

use primes::{is_prime, PrimeSet, Sieve};
use std::time::Instant;

fn longest_chain(longest_known: usize, start: usize, limit: usize) -> (usize, usize) {
    let mut longest: usize = 0;
    let mut chain_prime: usize = 0;
    let mut sum: usize = 0;
    let mut pset = Sieve::new();
    for (idx, prime) in pset.iter().enumerate().skip(start) {
        sum += prime as usize;
        let foo: i64 = longest_known as i64 + 1 + start as i64 - idx as i64;
        if sum >= limit || prime as i64 * foo >= limit as i64 {
            return (chain_prime.try_into().unwrap(), longest);
        }

        if is_prime(sum.try_into().unwrap()) && idx - start + 1 > longest {
            longest = idx - start + 1;
            chain_prime = sum;
        }
    }

    (chain_prime, longest)
}

fn find_longest(limit: usize) -> (usize, usize) {
    let mut longest: usize = 0;
    let mut longest_prime: usize = 0;
    for start in 0.. {
        if start * longest >= limit {
            break;
        }
        let (prime, found) = longest_chain(longest, start, limit);

        if found > longest {
            longest = found;
            longest_prime = prime;
        }
    }

    (longest, longest_prime)
}

#[test]
fn test_find_largest_under() {
    assert_eq!(find_longest(100), (6, 41));
    assert_eq!(find_longest(1000), (21, 953));
}

fn main() {
    let start = Instant::now();
    let solution = find_longest(1_000_000);
    println!("Solution: {:?}", solution);
    println!("Time elapsed: {}", start.elapsed().as_secs_f64());
}
