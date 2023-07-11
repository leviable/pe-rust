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

use itertools::Itertools;
use primes::{is_prime, PrimeSet, Sieve};
use std::collections::{HashMap, HashSet};

fn concats_to_prime(prime1: u64, prime2: u64) -> bool {
    is_prime(
        [prime1, prime2]
            .iter()
            .map(|x| x.to_string())
            .collect::<String>()
            .parse()
            .unwrap(),
    ) && is_prime(
        [prime2, prime1]
            .iter()
            .map(|x| x.to_string())
            .collect::<String>()
            .parse()
            .unwrap(),
    )
}

#[test]
fn test_concats_to_prime() {
    assert!(concats_to_prime(3, 7));
    assert!(concats_to_prime(3, 137));
    assert!(!concats_to_prime(7, 137));
    assert!(concats_to_prime(3, 229));
    assert!(concats_to_prime(7, 229));
    assert!(!concats_to_prime(137, 229));
}

fn prime_pair_sets(len: u32) -> Vec<u32> {
    let mut min = 1_000_000;
    let mut hm: HashMap<u64, HashSet<u64>> = HashMap::new();
    'bigloop: for (idx, prime1) in Sieve::new().iter().enumerate() {
        if prime1 > 674 {
            break;
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
            let mut found_one = false;
            for perm in v.iter().permutations(len as usize) {
                // println!("222222222222222222222222222222222222222222");
                // println!("{:?}", perm);
                let perm_copy = perm.clone();
                if &hm
                    .get(perm[0])
                    .unwrap()
                    .intersection(hm.get(perm[1]).unwrap())
                    .into_iter()
                    .collect::<Vec<_>>()
                    .len()
                    >= &4
                    && &hm
                        .get(perm[0])
                        .unwrap()
                        .intersection(hm.get(perm[2]).unwrap())
                        .into_iter()
                        .collect::<Vec<_>>()
                        .len()
                        >= &4
                    && &hm
                        .get(perm[0])
                        .unwrap()
                        .intersection(hm.get(perm[3]).unwrap())
                        .into_iter()
                        .collect::<Vec<_>>()
                        .len()
                        >= &4
                // && &hm
                //     .get(perm[0])
                //     .unwrap()
                //     .intersection(hm.get(perm[4]).unwrap())
                //     .into_iter()
                //     .collect::<Vec<_>>()
                //     .len()
                //     >= &5
                {
                    found_one = true;
                    let mut s = 0;
                    for x in perm_copy {
                        s += x;
                    }

                    if s < min {
                        println!("0000000000000000000000000000000000000000000");
                        println!("Found one for {k}:\n{:?} {min}", perm);
                        println!("1111111111111111111111111111111111111111111");
                        println!("{:?}", &hm.get(k));
                        min = s;
                    }
                }
            }
            // if found_one {
            //     break 'bigloop;
            // }
        }
    }
    // println!("{:?}", hm);
    vec![]
}

#[test]
fn test_prime_pair_sets() {
    assert_eq!(prime_pair_sets(4), vec![3, 7, 109, 673]);
}

fn pe060() -> u64 {
    prime_pair_sets(4);
    0
}

fn main() {
    let start = std::time::Instant::now();
    let solution = pe060();
    println!("Solution: {solution}");
    println!("Time elapsed: {}", start.elapsed().as_secs_f64());
}
