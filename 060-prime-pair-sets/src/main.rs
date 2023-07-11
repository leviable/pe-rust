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

fn prime_pair_sets(len: u32) -> Vec<u64> {
    let mut min: Option<u64> = None;
    let mut min_vec: Vec<u64> = vec![];
    let mut prime_vec = Sieve::new().iter().take(300).collect_vec();
    // Do in an increasing order of permutations: perms(2), perms(3), etc, whittle down
    'loopy: for perm in prime_vec.iter().combinations(len as usize) {
        for perm2 in perm.clone().iter().permutations(2) {
            if !concats_to_prime(**perm2[0], **perm2[1]) {
                continue 'loopy;
            }
        }
        // println!("Perm2 to primes: {:?}", perm2);
        let mut perm_sum = 0;
        for x in perm.clone() {
            perm_sum += x;
        }
        // println!("Perm2 to sum: {}", perm_sum);
        match min {
            Some(x) => {
                if perm_sum < x {
                    println!("This one is less: {} {perm_sum} for {:?}", min.unwrap(), perm);
                    min = Some(perm_sum);
                    min_vec = vec![];
                    for p in &perm {
                        min_vec.push(**p);
                    }
                    min_vec.sort();
                }
            }
            None => {
                println!("Initializing: {} for {:?}", perm_sum, perm);
                min = Some(perm_sum);
                min_vec = vec![];
                for p in &perm {
                    min_vec.push(**p);
                }
                min_vec.sort();
            }
        }
    }

    min_vec
}

#[test]
fn test_prime_pair_sets() {
    assert_eq!(prime_pair_sets(2), vec![3, 7]);
    assert_eq!(prime_pair_sets(3), vec![3, 37, 67]);
    assert_eq!(prime_pair_sets(4), vec![3, 7, 109, 673]);
}

fn pe060() -> u64 {
    prime_pair_sets(3).iter().sum()
}

fn main() {
    let start = std::time::Instant::now();
    let solution = pe060();
    println!("Solution: {solution}");
    println!("Time elapsed: {}", start.elapsed().as_secs_f64());
}
