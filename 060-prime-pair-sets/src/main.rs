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
use std::collections::{HashMap, HashSet};

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

    return is_prime(p1) && is_prime(p2);
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

fn all_prime(candidates: Vec<&u64>) -> bool {
    for combo in candidates.iter().combinations(2) {
        if !concats_to_prime(**combo[0], **combo[1]) {
            return false;
        }
    }
    true
}

#[test]
fn test_all_prime() {
    assert!(all_prime(vec![&3, &7]));
    assert!(all_prime(vec![&3, &37, &67]));
    assert!(all_prime(vec![&3, &7, &109, &673]));
    assert!(!all_prime(vec![&3, &77, &109, &673]));
}

fn prime_pair_sets(prime_vec: Vec<u64>, level: u64, limit: u64) -> Vec<u64> {
    // Whittle down by combination level
    let mut new_prime_set: HashSet<u64> = HashSet::new();

    for combo in prime_vec
        .iter()
        .combinations(level as usize)
        .filter(|y| all_prime(y.clone()))
    {
        new_prime_set.extend(combo);
    }

    println!("555555555555555555555555555555555555555555555555555");
    println!("new_prime_set: {:?}", new_prime_set);

    if level == limit {
        println!("999999999999999999999999999999999999999999999999");
        println!(
            "Prime set: {:?}",
            new_prime_set.clone().iter().sorted().collect::<Vec<_>>()
        );
        let mut min = 1_000_000;
        let mut min_vec = vec![];
        for prime_combo in new_prime_set.iter().sorted().combinations(level as usize) {
            let new_sum = prime_combo.iter().map(|x| **x).sum();
            if new_sum < min {
                println!("8888888888888888888888888888888888888888888");
                println!("New min: {min} {new_sum} {:?}", prime_combo.clone());
                min = new_sum;
                min_vec = prime_combo.clone();
            }
        }
        return min_vec.iter().map(|x| **x).collect();
    } else {
        println!("7777777777777777777777777777777777777777777777777777");
        return prime_pair_sets(
            new_prime_set.into_iter().sorted().collect(),
            level + 1,
            limit,
        );
    }
}

#[test]
fn test_prime_pair_sets() {
    let prime_vec = Sieve::new().iter().take(300).collect_vec();
    assert_eq!(prime_pair_sets(prime_vec.clone(), 2, 2), vec![3, 7]);
    assert_eq!(prime_pair_sets(prime_vec.clone(), 2, 3), vec![3, 37, 67]);
    assert_eq!(
        prime_pair_sets(prime_vec.clone(), 2, 4),
        vec![3, 7, 109, 673]
    );
}

fn pe060() -> u64 {
    let prime_vec = Sieve::new().iter().take(300).collect_vec();
    prime_pair_sets(prime_vec, 2, 5).iter().sum()
}

fn main() {
    let start = std::time::Instant::now();
    let solution = pe060();
    println!("Solution: {solution}");
    println!("Time elapsed: {}", start.elapsed().as_secs_f64());
}
