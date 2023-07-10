// The primes 3, 7, 109, and 673, are quite remarkable. By taking any two
// primes and concatenating them in any order the result will always be prime.
// For example, taking 7 and 109, both 7109 and 1097 are prime. The sum of these
// four primes, 792, represents the lowest sum for a set of four primes with
// this property.
//
// Find the lowest sum for a set of five primes for which any two primes
// concatenate to produce another prime.

use itertools::Itertools;
use primes::{is_prime, PrimeSet, Sieve};

fn perms_are_prime(primes: &Vec<u64>) -> bool {
    primes
        .iter()
        .permutations(2)
        .map(|x| {
            x.iter()
                .map(|y| y.to_string())
                .collect::<String>()
                .parse()
                .unwrap()
        })
        .collect::<Vec<u64>>()
        .iter()
        .all(|z| is_prime(*z))
}

#[test]
fn test_perms_are_prime() {
    assert!(perms_are_prime(&vec![3, 7, 109, 673]));
    assert!(!perms_are_prime(&vec![2, 7, 109, 673]));
}

fn pe060() -> u64 {
    let mut lowest = 1_000_000_000_000u64;
    'loop1: for (p1_idx, p1) in Sieve::new().iter().skip(1).enumerate() {
        for (p2_idx, p2) in Sieve::new().iter().skip(p1_idx + 2).enumerate() {
            for (p3_idx, p3) in Sieve::new().iter().skip(p2_idx + 3).enumerate() {
                for (p4_idx, p4) in Sieve::new().iter().skip(p3_idx + 4).enumerate() {
                    for p5 in Sieve::new().iter().skip(p4_idx + 5) {
                        let candidates = vec![p1, p2, p3, p4, p5];
                        println!("Checking: {:?}", candidates);
                        if !perms_are_prime(&candidates) {
                            continue;
                        }

                        let sum = candidates.iter().sum();

                        if sum < lowest {
                            println!("Found one: {sum}");
                            lowest = sum;
                            break;
                        }
                    }
                }
            }
        }
    }
    0
}

fn main() {
    let start = std::time::Instant::now();
    let solution = pe060();
    println!("Solution: {solution}");
    println!("Time elapsed: {}", start.elapsed().as_secs_f64());
}
