// The primes 3, 7, 109, and 673, are quite remarkable. By taking any two
// primes and concatenating them in any order the result will always be prime.
// For example, taking 7 and 109, both 7109 and 1097 are prime. The sum of these
// four primes, 792, represents the lowest sum for a set of four primes with
// this property.
//
// Find the lowest sum for a set of five primes for which any two primes
// concatenate to produce another prime.
//
// NOTE: Two observations:
// 1 -> I am checking and rechecking a lot of possible concatenated primes multiple
//      multiple times. I should probably be looking up in some cache table or,
//      even better, just iterating over the ones I already know are invalid.
// 2 -> It might be a better approach to start counting primes and destructuring
//      them. But that doesn't tell me about all the numbers. If I destructure
//      7109 and deduce 7 and 109, that tells me nothing about 3 and 673
// 3 -> Maybe it would be best to do all the associations at once for some given
//      range, and then try and filter down?

use itertools::Itertools;
use primes::{is_prime, PrimeSet, Sieve};
// use std::collections::{HashMap, HashSet};

fn concats_to_prime(prime1: u64, prime2: u64) -> bool {
    let p1 = [prime1, prime2]
        .iter()
        .map(|x| x.to_string())
        .collect::<String>()
        .parse()
        .unwrap();
    let p2 = [prime2, prime1]
        .iter()
        .map(|x| x.to_string())
        .collect::<String>()
        .parse()
        .unwrap();

    is_prime(p1) && is_prime(p2)
}

#[test]
fn test_concats_to_prime() {
    assert!(concats_to_prime(3, 7));
    assert!(concats_to_prime(3, 137));
    assert!(!concats_to_prime(7, 137));
    assert!(concats_to_prime(3, 229));
    assert!(concats_to_prime(7, 229));
    assert!(!concats_to_prime(137, 229));
    assert!(!concats_to_prime(3, 19));
    assert!(!concats_to_prime(2, 19));
    assert!(!concats_to_prime(2, 2));
}

fn all_prime(candidates: &[u64]) -> bool {
    for combo in candidates.iter().combinations(2) {
        if !concats_to_prime(*combo[0], *combo[1]) {
            return false;
        }
    }
    true
}

#[test]
fn test_all_prime() {
    assert!(all_prime(&vec![3, 7]));
    assert!(all_prime(&vec![3, 37, 67]));
    assert!(all_prime(&vec![3, 7, 109, 673]));
    assert!(!all_prime(&vec![3, 77, 109, 673]));
}

fn prime_pair_sets(
    primes: Vec<u64>,
    candidates: Vec<Vec<u64>>,
    level: u64,
    limit: usize,
) -> Vec<u64> {
    println!("000000000000000000000000000000000000000000000000000000000000000000");
    println!("level: {level}");
    println!("c cnt: {}", &candidates.len() + 1);
    // Whittle down by combination level
    let mut new_candidates = vec![];
    for (idx, candidate) in candidates.iter().enumerate() {
        println!("33333333333333333333333333333333333");
        println!("Working on {:?}", candidate.clone());
        for prime in primes.clone().iter().skip(idx + level as usize) {
            let mut c = candidate.clone();
            c.push(*prime);
            if all_prime(&c) {
                println!("Updating to {:?}", c.clone());
                new_candidates.push(c);
            }
        }
    }
    if level == limit as u64 {
        let mut min = 1_000_000_000;
        let mut min_vec = vec![];
        for candidate in new_candidates {
            println!("2222222222222222222222222222222222222222222");
            println!("Working on {:?}", candidate.clone());
            let sum = candidate.clone().iter().sum();
            if sum < min {
                println!("11111111111111111111111111111111111");
                println!("New min: {:?} {sum} {min}", candidate.clone());
                min = sum;
                min_vec = candidate.clone();
            }
        }
        min_vec
    } else {
        prime_pair_sets(primes, new_candidates, level + 1, limit)
    }
}

#[test]
fn test_prime_pair_sets() {
    let prime_vec = Sieve::new().iter().take(125).collect_vec();
    let candidates = prime_vec.clone().into_iter().map(|x| vec![x]).collect_vec();
    assert_eq!(
        prime_pair_sets(prime_vec.clone(), candidates.clone(), 2, 2),
        vec![3, 7]
    );
    assert_eq!(
        prime_pair_sets(prime_vec.clone(), candidates.clone(), 2, 3),
        vec![3, 37, 67]
    );
    assert_eq!(
        prime_pair_sets(prime_vec.clone(), candidates.clone(), 2, 4),
        vec![3, 7, 109, 673]
    );
}

fn pe060() -> u64 {
    let count = 5000;
    let level = 5;
    let prime_vec = Sieve::new().iter().take(count).collect_vec();
    let candidates = prime_vec
        .clone()
        .into_iter()
        .take(count - level)
        .map(|x| vec![x])
        .collect_vec();
    prime_pair_sets(prime_vec, candidates, 2, level)
        .iter()
        .sum()
}

fn main() {
    let start = std::time::Instant::now();
    let solution = pe060();
    println!("Solution: {solution}");
    println!("Time elapsed: {}", start.elapsed().as_secs_f64());
}
