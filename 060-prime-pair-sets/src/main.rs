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

    is_prime(p1) && is_prime(p2)
}

#[test]
fn test_concats_to_prime() {
    assert!(concats_to_prime(3, 7));
    assert!(concats_to_prime(3, 137));
    assert!(concats_to_prime(3, 229));
    assert!(concats_to_prime(7, 229));
    assert!(!concats_to_prime(7, 137));
    assert!(!concats_to_prime(137, 229));
    assert!(!concats_to_prime(3, 19));
    assert!(!concats_to_prime(2, 19));
    assert!(!concats_to_prime(2, 2));
}

fn all_prime(candidates: &[u64]) -> bool {
    candidates
        .iter()
        .combinations(2)
        .all(|x| concats_to_prime(*x[0], *x[1]))
}

#[test]
fn test_all_prime() {
    assert!(all_prime(&vec![3, 7]));
    assert!(all_prime(&vec![3, 37, 67]));
    assert!(all_prime(&vec![3, 7, 109, 673]));
    assert!(!all_prime(&vec![3, 77, 109, 673]));
    assert!(!all_prime(&vec![3, 17, 41]));
    assert!(!all_prime(&vec![3, 7, 13]));
}

fn prime_pair_sets(primes: HashSet<u64>, dgts: u64) -> Vec<u64> {
    // NOTE:
    // next attempt: For every prime under X, add prime + set for all the
    // other primes that it can all_prime with.
    // Then do a combo of dgts spots, and see if they all intersect with
    // dgts number of members
    println!("000000000000000000000000000000000000000000000000000000000000000000");

    let mut hm: HashMap<u64, HashSet<u64>> = HashMap::new();

    for (idx, prime) in primes.clone().into_iter().enumerate() {
        for other in primes.clone().iter() {
            if concats_to_prime(prime, *other) {
                let entry = hm.entry(prime).or_insert(HashSet::new());
                entry.insert(*other);
            }
        }
    }

    for x in 3..6 {
        println!("Doing combo: {x}");
        'combo: for combo in hm.clone().keys().combinations(x) {
            for smaller_combo in combo.iter().combinations(2) {
                if !hm.contains_key(smaller_combo[0]) || !hm.contains_key(smaller_combo[1]) {
                    continue 'combo;
                }
                let inter = hm[smaller_combo[0]]
                    .intersection(&hm[smaller_combo[1]])
                    .collect::<Vec<_>>();
                if inter.len() == 0 {
                    continue;
                }
                if inter.len() < 4 {
                    println!("5555555555555555555555555555555555555555555555555");
                    println!("{:?}", inter);
                    hm.remove(smaller_combo[0]);
                    continue 'combo;
                }
            }
        }
    }

    for (k, v) in hm.iter() {
        println!("{k}:\n{:?}", v);
    }

    println!("111111111111111111111111111111111111111111111111111111111111111111");
    println!("111111111111111111111111111111111111111111111111111111111111111111");
    println!("111111111111111111111111111111111111111111111111111111111111111111");

    vec![]
}

#[test]
fn test_prime_pair_sets() {
    // let prime_set = HashSet::from_iter(Sieve::new().iter().take(125));
    // assert_eq!(prime_pair_sets(prime_set.clone(), 2), vec![3, 7]);
    // assert_eq!(prime_pair_sets(prime_set.clone(), 3), vec![3, 37, 67]);
    // assert_eq!(prime_pair_sets(prime_set.clone(), 4), vec![3, 7, 109, 673]);
}

fn pe060() -> u64 {
    let count = 500;
    let dgts = 5;
    let prime_set = HashSet::from_iter(Sieve::new().iter().take(count));
    prime_pair_sets(prime_set, dgts).iter().sum()
}

fn main() {
    let start = std::time::Instant::now();
    let solution = pe060();
    println!("Solution: {solution}");
    println!("Time elapsed: {}", start.elapsed().as_secs_f64());
}
