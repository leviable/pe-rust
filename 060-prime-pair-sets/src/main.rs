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
    // println! {"333333333333333333333333333333333333333333333333333333333"};
    // println! {"{prime1}, {prime2}"};
    // println! {"333333333333333333333333333333333333333333333333333333333"};
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

fn pair_set(primes: &Vec<u64>, size: usize) -> bool {
    // println!("44444444444444444444444444444444444444444444444");
    // println!("44444444444444444444444444444444444444444444444");
    // println!("44444444444444444444444444444444444444444444444");

    if size == 2 {
        return true;
    }
    let mut accumulator = primes.clone();
    let mut answer: Vec<u64> = vec![];
    for combo_size in 3..=size {
        let working_vec = accumulator;
        accumulator = vec![];
        for combo in working_vec.iter().permutations(combo_size) {
            if combo
                .iter()
                .combinations(2)
                .all(|x| concats_to_prime(**x[0], **x[1]))
            {
                answer.extend(combo.clone());
                accumulator.extend(combo.clone());
                println!("Extending accumulator with: {:?}", combo);
                println!("Accumulator is            : {:?}", accumulator);
            }
        }
    }

    let answer_hashset: HashSet<u64> = HashSet::from_iter(answer.into_iter());
    let mut answer_vec = answer_hashset.into_iter().collect::<Vec<_>>();
    // let answer: Vec<&u64> = HashSet::from_iter(answer.into_iter())
    //     .iter()
    //     .to_owned()
    //     .collect::<Vec<_>>();
    answer_vec.sort();
    println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@");
    println!("Answer vec : {:?}", answer_vec);
    println!("Answer size: {}", answer_vec.len());
    println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@");
    if answer_vec.len() >= size - 1 {
        println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
        println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
        println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
        true
    } else {
        false
    }
}

fn prime_pair_sets(size: usize) -> Vec<u64> {
    // Get an empty hashmap for primes and their concat pairs
    // for every prime after 2
    //      - for every (key, prime_container) in hashmap
    //          - if prime and key is concat prime, add to prime to key's container
    //          - if check_for_pair_set(key container)
    //              - Return set
    let mut hm: HashMap<u64, Vec<u64>> = HashMap::new();

    for prime in Sieve::new().iter().skip(1) {
        // println!("111111111111111111111111111111111111111111111111111111");
        // println!("Working on prime: {}", prime);
        // println!("Hashmap: {:?}", hm.clone());
        // println!("111111111111111111111111111111111111111111111111111111");
        if prime > 700 {
            break;
        }

        for (k, v) in hm.clone() {
            // println!("555555555555555555555555555555555555555555555555");
            // println!("555555555555555555555555555555555555555555555555");
            // println!("555555555555555555555555555555555555555555555555");

            if concats_to_prime(k, prime) {
                // println!("222222222222222222222222222222222222222222222222");
                // println!("Concats to prime: {k}, {prime}");
                // println!("222222222222222222222222222222222222222222222222");
                let entry = hm.entry(k).or_insert(vec![]);
                entry.push(prime);

                if pair_set(&entry.clone(), size) {
                    println!("66666666666666666666666666666666666666666666666666666");
                    println!("{:?} {:?}", k, entry);
                    println!("66666666666666666666666666666666666666666666666666666");
                    let mut foo = vec![];
                    foo.push(k);
                    foo.extend(entry.clone());
                    return foo;
                }
            }
        }

        hm.insert(prime, vec![]);
    }

    vec![]
}

#[test]
fn test_prime_pair_sets() {
    assert_eq!(prime_pair_sets(2), vec![3, 7]);
    assert_eq!(prime_pair_sets(4), vec![3, 7, 109, 673]);
}

fn pe060() -> u64 {
    prime_pair_sets(5).iter().sum()
}

fn main() {
    let start = std::time::Instant::now();
    let solution = pe060();
    println!("Solution: {solution}");
    println!("Time elapsed: {}", start.elapsed().as_secs_f64());
}
