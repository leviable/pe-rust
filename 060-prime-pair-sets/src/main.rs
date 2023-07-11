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
    let mut min = 1_000_000;
    let mut min_vec: Vec<u64> = vec![];
    let mut hm: HashMap<u64, HashSet<u64>> = HashMap::new();
    'bigloop: for (idx, prime1) in Sieve::new().iter().enumerate() {
        if idx % 25 == 0 {
            println!("Working on idx {idx}: {prime1}");
        }
        for prime2 in Sieve::new().iter().take(idx) {
            if concats_to_prime(prime1.clone(), prime2.clone()) {
                let entry = &mut hm.entry(prime1.clone()).or_insert(HashSet::from([prime1]));
                entry.insert(prime2.clone());
                let entry = &mut hm.entry(prime2.clone()).or_insert(HashSet::from([prime2]));
                entry.insert(prime1.clone());
            }
        }
        'forloop: for (k, v) in &hm {
            if v.len() < len as usize {
                continue;
            }
            'loopfoo: for perm in v.iter().permutations(len as usize) {
                for perm2 in perm.iter().permutations(2) {
                    if &hm
                        .get(perm2[0])
                        .unwrap()
                        .intersection(hm.get(perm2[1]).unwrap())
                        .into_iter()
                        .collect::<Vec<_>>()
                        .len()
                        < &(len as usize)
                    {
                        continue 'loopfoo;
                    }
                }
                let mut sum = 0;
                for foo in &perm {
                    sum += *foo;
                }
                if sum < min {
                    println!("5555555555555555555555555555555555555555555");
                    println!("Maybe found one? {:?} {sum} {min}", perm);
                    min = sum;
                    let mut foo: Vec<u64> = vec![];
                    for x in perm.iter() {
                        foo.push(**x);
                    }
                    foo.sort();
                    min_vec = foo;
                    min_vec.sort();
                }
            }
        }
        // if min_vec.len() > 0 && prime1 > min_vec[min_vec.len() - 1] {
        //     return min_vec;
        // }
    }
    dbg!(hm);
    vec![]
}

#[test]
fn test_prime_pair_sets() {
    assert_eq!(prime_pair_sets(2), vec![3, 7]);
    assert_eq!(prime_pair_sets(3), vec![3, 37, 67]);
    // assert_eq!(prime_pair_sets(4), vec![3, 7, 109, 673]);
}

fn pe060() -> u64 {
    prime_pair_sets(5);
    0
}

fn main() {
    let start = std::time::Instant::now();
    let solution = pe060();
    println!("Solution: {solution}");
    println!("Time elapsed: {}", start.elapsed().as_secs_f64());
}
