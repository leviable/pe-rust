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

fn find_largest(limit: u64) -> u64 {
    let mut largest = 0;
    let mut sum = 0;
    for (idx, prime) in Sieve::new().iter().enumerate() {
        sum += prime;
        println!("Now working on: #{idx} {prime} {largest} {sum}");
        if sum >= limit {
            println!("Sum {sum} is >= limit {limit}");
            break;
        }
        if is_prime(sum) && sum > largest {
            println!("Sum {sum} is > {largest}");
            largest = sum
        }
    }

    largest
}

#[test]
fn test_find_largest_under() {
    assert_eq!(find_largest(100), 41);
    assert_eq!(find_largest(1000), 953);
}

fn main() {
    let start = Instant::now();
    println!("Time elapsed: {}", start.elapsed().as_secs_f64());
}
