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

fn find_longest(skip: usize, limit: u64) -> Option<u64> {
    let mut longest = 0;
    let mut sum = 0;

    if skip % 1000 == 0 {
        println!("Passing {skip}");
    }

    for (idx, prime) in Sieve::new().iter().enumerate() {
        if idx > 200_000 || skip >= limit.try_into().unwrap() {
            return Some(0);
        }
        if idx < skip {
            continue;
        }
        sum += prime;
        if sum >= limit {
            break;
        }
        if is_prime(sum) && idx - skip + 1 > longest {
            // println!("Sum {sum} Prime {prime} Idx {}", idx - skip + 1);
            longest = idx - skip + 1;
        }
    }
    let next_longest = find_longest(skip + 1, limit);
    if next_longest < Some(longest.try_into().unwrap()) {
        Some(longest.try_into().unwrap())
    } else {
        next_longest
    }
}

#[test]
fn test_find_largest_under() {
    assert_eq!(find_longest(0, 100), Some(6));
    assert_eq!(find_longest(0, 1000), Some(21));
    panic!("121111111111111111111");
}

fn main() {
    let start = Instant::now();
    let solution = find_longest(0, 1_000_000);
    println!("Solution: {}", solution.unwrap());
    println!("Time elapsed: {}", start.elapsed().as_secs_f64());
}
