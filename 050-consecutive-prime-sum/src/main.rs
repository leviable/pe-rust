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

fn longest_chain(
    longest_known_chain_count: usize,
    chain_start: usize,
    prime_limit: usize,
) -> (usize, usize) {
    let mut longest: usize = 0;
    let mut chain_prime: usize = 0;
    let mut sum: usize = 0;
    let mut pset = Sieve::new();
    for (idx, prime) in pset.iter().enumerate().skip(chain_start) {
        // Even though we call ".skip()", the idx will be as if we hadn't skipped any.
        // Need to make sure to subtract chain_start from idx for actual chain size
        sum += prime as usize;
        // If the minimum number of remaining chain slots * the next prime in the chain
        // is greater than our prime limit, then there is no point in doing further
        // calculations; break out
        let minimum_needed: i64 =
            longest_known_chain_count as i64 + 1 + chain_start as i64 - idx as i64;
        if sum >= prime_limit || prime as i64 * minimum_needed >= prime_limit as i64 {
            return (chain_prime, longest);
        }

        if is_prime(sum.try_into().unwrap()) && idx - chain_start + 1 > longest {
            longest = idx - chain_start + 1;
            chain_prime = sum;
        }
    }

    (chain_prime, longest)
}

fn find_longest_chain(prime_limit: usize) -> (usize, usize) {
    let mut longest_chain_count: usize = 0;
    let mut longest_chain_prime: usize = 0;
    for start in 0.. {
        if start * longest_chain_count >= prime_limit {
            break;
        }
        let (chain_prime, chain_count) = longest_chain(longest_chain_count, start, prime_limit);

        if chain_count > longest_chain_count {
            longest_chain_count = chain_count;
            longest_chain_prime = chain_prime;
        }
    }

    (longest_chain_count, longest_chain_prime)
}

#[test]
fn test_find_largest_under() {
    assert_eq!(find_longest_chain(100), (6, 41));
    assert_eq!(find_longest_chain(1000), (21, 953));
}

fn main() {
    let start = Instant::now();
    let solution = find_longest_chain(1_000_000);
    println!("Solution: {:?}", solution);
    println!("Time elapsed: {}", start.elapsed().as_secs_f64());
}
