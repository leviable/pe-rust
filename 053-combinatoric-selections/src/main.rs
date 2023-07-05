// There are exactly ten ways of selecting three from five, 12345:
//
//       123, 124, 125, 134, 135, 145, 234, 235, 245, and 345
//
// In combinatorics, we use the notation, (5|3) = 10 .
//
// In general, (n | r) =     n!
//                        --------
//                        r!(n-r)!
//
// , where r <= n , and n! = n * (n-1) * (n-2) ... * 3 * 2 * 1
//
// It is not until n = 23 that a value exceeds one-million: (23 | 10) = 1144066
//
// How many, not necessarily distinct, values of (n|r) for 1 <= n <= 100,
// are greater than one-million?

use primes::{PrimeSet, Sieve};
use std::collections::HashMap;
use std::time::Instant;

fn factorial(num: u64) -> Vec<u64> {
    let mut v = vec![];
    for f in 2..=num {
        v.push(f);
    }
    v.reverse();
    v
}

#[test]
fn test_factorial() {
    assert_eq!(factorial(10), vec![10, 9, 8, 7, 6, 5, 4, 3, 2]);
    assert_eq!(factorial(5), vec![5, 4, 3, 2]);
}

fn common_denoms(num: u64) -> HashMap<u64, u64> {
    let mut denoms: HashMap<u64, u64> = HashMap::new();
    let mut val = num;
    for prime in Sieve::new().iter() {
        if prime > val {
            break;
        }
        loop {
            if val % prime == 0 {
                let count = denoms.entry(prime).or_insert(0);
                *count += 1;
                val /= prime;
            } else {
                break;
            }
        }
    }

    denoms
}

#[test]
fn test_common_denoms() {
    assert_eq!(common_denoms(10), HashMap::from([(2u64, 1), (5, 1)]));
    assert_eq!(common_denoms(17), HashMap::from([(17u64, 1),]));
    assert_eq!(common_denoms(8), HashMap::from([(2u64, 3),]));
}

fn combine_hashmaps(maps: Vec<HashMap<u64, u64>>) -> HashMap<u64, u64> {
    let mut return_map: HashMap<u64, u64> = HashMap::new();

    for map in maps {
        for (k, v) in map.iter() {
            let count = return_map.entry(*k).or_insert(0);
            *count += v;
        }
    }

    return_map
}

#[test]
fn test_combine_hashmaps() {
    let hm1: HashMap<u64, u64> = HashMap::from_iter([(2, 1), (3, 1), (5, 1)]);
    let hm2: HashMap<u64, u64> = HashMap::from_iter([(2, 1), (5, 1), (7, 1)]);
    let expect: HashMap<u64, u64> = HashMap::from_iter([(2, 2), (3, 1), (5, 2), (7, 1)]);
    assert_eq!(combine_hashmaps(vec![hm1, hm2]), expect);
}

fn combinate(n: u64, r: u64) -> u64 {
    let mut top = combine_hashmaps(
        factorial(n)
            .iter()
            .map(|x| common_denoms(*x))
            .collect::<Vec<_>>(),
    );

    let r_fac = factorial(r);
    let n_minus_r_fac = factorial(n - r);

    let mut r_fac_n_minus_r_fac = r_fac;
    r_fac_n_minus_r_fac.extend(n_minus_r_fac);
    let bottom = combine_hashmaps(
        r_fac_n_minus_r_fac
            .iter()
            .map(|x| common_denoms(*x))
            .collect::<Vec<_>>(),
    );
    // let mut num = 1u64;
    let mut den = 1u64;
    for (k, mut v) in bottom {
        // println!("4444444444444444444444444444444444444444");
        // println!("k: {k}, v: {v}");
        loop {
            let mut count = top.entry(k).or_insert(0);
            match count {
                0 => break,
                _ => {
                    if v > *count {
                        println!("5555555555555555555555555555555555555555");
                        println!("count: {count}, v: {v}");
                    }
                    // println!("3333333333333333333333333333333333333333");
                    // println!("count: {count}, v: {v}");
                    *count -= 1;
                    v -= 1
                }
            };
            if v == 0 {
                break;
            }
        }

        if v > 0 {
            println!("22222222222222222222222222222222222222");
            println!("Den was {den}");
            den *= v;
            println!("Den is {den}");
        }
    }

    let mut result = 1.0 / den as f64;
    // println!("1111111111111111111111111111111111111");
    // println!("result: {result} {den}");

    for (k, v) in top {
        if v > 0 {
            result *= (k * v) as f64;
        }

        if result >= 1_000_000.0 {
            return 1_000_001;
        }
    }

    result as u64
}

#[test]
fn test_combinate() {
    // assert_eq!(combinate(23, 10), 1144066);
    assert_eq!(combinate(23, 10), 1_000_001);
    assert_eq!(combinate(5, 3), 10);
}

fn pe053() -> u64 {
    let mut over_1_million = 0u64;
    for n in 1..=100 {
        for r in 1..=n {
            if combinate(n, r) >= 1_000_000 {
                over_1_million += 1
            }
        }
    }
    over_1_million
}

fn main() {
    let start = Instant::now();

    println!("PE-053: {}", pe053());
    println!("Time elapsed: {}", start.elapsed().as_secs_f64());
}
