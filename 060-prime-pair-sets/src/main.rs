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
    assert!(concats_to_prime(3, 17));
    assert!(concats_to_prime(3, 137));
    assert!(concats_to_prime(3, 229));
    assert!(concats_to_prime(7, 229));
    assert!(concats_to_prime(3, 37));
    assert!(concats_to_prime(3, 67));
    assert!(concats_to_prime(37, 67));
    assert!(!concats_to_prime(7, 137));
    assert!(!concats_to_prime(137, 229));
    assert!(!concats_to_prime(3, 19));
    assert!(!concats_to_prime(2, 19));
    assert!(!concats_to_prime(2, 2));
}

fn pair_set(primes: &Vec<u64>, size: usize) -> Option<Vec<u64>> {
    if size == 1 {
        let mut primes = primes.clone().to_owned();
        primes.sort();
        return Some(primes);
    }

    let mut answers = vec![];
    let mut candidates = HashSet::from_iter(primes.iter());

    for permu_size in 2..=size {
        let permu_candidates = candidates;
        candidates = HashSet::new();
        for permu in permu_candidates.into_iter().permutations(permu_size) {
            if permu
                .clone()
                .iter()
                .combinations(2)
                .all(|x| concats_to_prime(**x[0], **x[1]))
            {
                candidates.extend(permu.clone());
                if permu_size == size {
                    answers.push(permu.clone().to_owned());
                }
            }
        }
    }

    if answers.len() > 0 {
        answers.iter().sorted_by(|x, y| x.cmp(y));
        let mut foo = answers
            .pop()
            .unwrap()
            .iter()
            .map(|x| **x)
            .collect::<Vec<u64>>();
        foo.sort();
        Some(foo)
    } else {
        None
    }
}

fn prime_pair_sets(size: usize) -> Option<Vec<u64>> {
    let mut hm: HashMap<u64, Vec<u64>> = HashMap::new();

    for (idx, prime) in Sieve::new().iter().skip(1).enumerate() {
        if idx % 100 == 0 {
            println!("111111111111111111111111111111111111111111111111111111");
            println!("Working on prime #{idx}: {prime}");
            println!("111111111111111111111111111111111111111111111111111111");
        }
        if prime > 15000 {
            break;
        }

        hm.insert(prime, vec![]);

        for (k, _) in hm.clone() {
            if concats_to_prime(k, prime) {
                let entry = hm.entry(k).or_insert(vec![]);
                entry.push(prime);

                if let Some(answer) = pair_set(&entry.clone(), size - 1) {
                    let mut foo = vec![];
                    foo.push(k);
                    foo.extend(answer);
                    return Some(foo);
                }
            }
        }

        hm.insert(prime, vec![]);
    }

    None
}

#[test]
fn test_prime_pair_sets() {
    assert_eq!(prime_pair_sets(2), Some(vec![3, 7]));
    assert_eq!(prime_pair_sets(3), Some(vec![3, 37, 67]));
    assert_eq!(prime_pair_sets(4), Some(vec![3, 7, 109, 673]));
}

fn pe060() -> u64 {
    prime_pair_sets(5).unwrap().iter().sum()
}

fn main() {
    let start = std::time::Instant::now();
    let solution = pe060();
    println!("Solution: {solution}");
    println!("Time elapsed: {}", start.elapsed().as_secs_f64());
}
