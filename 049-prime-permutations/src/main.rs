// The arithmetic sequence, 1487, 4817, 8147, in which each of the terms increases by 3330,
// is unusual in two ways:
//  (i)  each of the three terms are prime, and,
//  (ii) each of the 4-digit numbers are permutations of one another.
//
// There are no arithmetic sequences made up of three 1-, 2-, or 3-digit primes,
// exhibiting this property, but there is one other 4-digit increasing sequence.
//
// What 12-digit number do you form by concatenating the three terms in this sequence?

use itertools::Itertools;
use primes::{is_prime, PrimeSet, Sieve};
use std::collections::HashSet;
use std::time::Instant;

fn get_perms(prime: u64, perm_cnt: usize) -> Vec<u64> {
    // println!("About to calculate perms for {prime}");
    prime
        .to_string()
        .chars()
        .permutations(perm_cnt)
        .unique()
        .collect::<Vec<_>>()
        .iter()
        .map(|x| x.iter().collect::<String>().parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}

fn filter_perms(perms: Vec<u64>) -> Vec<u64> {
    let mut filtered = vec![];
    for p in perms {
        if is_prime(p) && p > 1_000 && p < 10_000 {
            filtered.push(p)
        }
    }

    filtered.sort();
    match filtered.len() {
        0..=2 => vec![],
        _ => filtered,
    }
}

#[test]
fn test_get_perms() {
    assert_eq!(get_perms(123, 3), vec![123, 132, 213, 231, 312, 321]);
}

fn has_property(candidates: &Vec<u64>) -> Option<Vec<u64>> {
    if candidates.len() < 2 {
        return None;
    }
    let first = candidates[0];
    for candidate in &candidates[1..] {
        let test_val = candidate + (candidate - &first);
        if candidates.contains(&test_val) {
            return Some(candidates.clone());
        }
    }
    return has_property(&candidates[1..].to_vec());
}

fn pe049() -> u64 {
    let mut h: HashSet<u64> = HashSet::new();
    let mut candidates: Vec<Vec<u64>> = vec![];
    for p in Sieve::new().iter().take(2000) {
        if p < 999 {
            continue;
        } else if p > 9_999 {
            break;
        }

        if h.contains(&p) {
            continue;
        }
        let perms = get_perms(p, 4);
        let filtered = filter_perms(perms);
        for f_perm in &filtered {
            h.insert(*f_perm);
        }
        if &filtered.len() < &3 {
            continue;
        }
        candidates.push(filtered);
    }

    for c in candidates {
        if let Some(found_one) = has_property(&c) {
            println!("33333333333333333333333333333333333333333333333333");
            println!("Found one: {:?}", found_one);
            println!("33333333333333333333333333333333333333333333333333");
        }
    }
    0
}

fn main() {
    let start = Instant::now();
    let solution = pe049();
    println!("Solution: {:?}", solution);
    println!("Time elapsed: {}", start.elapsed().as_secs_f64());
}
