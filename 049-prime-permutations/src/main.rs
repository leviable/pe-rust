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

fn get_permutations(num: u64) -> Vec<u64> {
    num.to_string()
        .chars()
        .permutations(4)
        .collect::<Vec<_>>()
        .iter()
        .map(|x| x.iter().collect::<String>().parse().unwrap())
        .filter(|y| is_prime(*y))
        .collect::<Vec<u64>>()
}

#[test]
fn test_get_permutations() {
    let expect = vec![1487, 1847, 4817, 4871, 8147, 8741, 7481, 7841];
    assert_eq!(get_permutations(1487), expect);
}

fn recurse(input: Vec<u64>) -> Option<Vec<u64>> {
    if input.len() < 3 {
        return None;
    }
    let (first, mid, last) = (
        input[0],
        &input[1..(input.len() - 1)],
        input[input.len() - 1],
    );
    if first > 999 {
        for val in mid {
            if last - val == val - first && last != *val && first != *val {
                // Found one!
                return Some(vec![first, *val, last]);
            }
        }
    }

    recurse(input[..input.len() - 1].to_vec())
}
// Some(vec![1488, 4817, 8147])

#[test]
fn test_recurse() {
    assert_eq!(recurse(vec!(1, 2)), None);
    let mut input = vec![1487, 1847, 4817, 4871, 8147, 8741, 7481, 7841];
    input.sort();
    assert_eq!(recurse(input), Some(vec![1487, 4817, 8147]));
}

fn has_property(num: u64) -> Option<Vec<u64>> {
    let mut found: Vec<u64> = vec![];
    for perm in get_permutations(num) {
        if !is_prime(perm) {
            continue;
        }
        if perm > 1000 {
            found.push(perm);
        }
    }

    let mut h: HashSet<u64> = HashSet::new();

    for x in &found {
        h.insert(*x);
    }

    let mut found: Vec<u64> = h.into_iter().collect();

    found.sort();

    if found.len() < 3 {
        return None;
    }

    // println!("1111111111111111111111111111111111111111");
    // println!("{:?}", &found);

    recurse(found)
}

#[test]
fn test_has_property() {
    assert_eq!(has_property(1489), None);
    assert_eq!(has_property(1487), Some(vec![1487, 4817, 8147]));
}
fn pe049() -> Vec<u64> {
    let h: HashSet<u64> = HashSet::new();
    for num in Sieve::new().iter() {
        if num < 1_000 {
            continue;
        } else if num > 9_999 {
            break;
        }

        match has_property(num) {
            Some(x) => {
                println!("4444444444444444444444444444444444444444444444");
                println!("Found one: {:?}", x);
                println!("4444444444444444444444444444444444444444444444");
            }
            None => continue,
        }
    }
    vec![]
}

fn main() {
    let start = Instant::now();
    let solution = pe049();
    println!("Solution: {:?}", solution);
    println!("Time elapsed: {}", start.elapsed().as_secs_f64());
}
