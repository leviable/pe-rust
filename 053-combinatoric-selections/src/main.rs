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

fn prime_divisors(num: u64) -> HashMap<u64, u64> {
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
fn test_prime_divisors() {
    assert_eq!(prime_divisors(10), HashMap::from([(2u64, 1), (5, 1)]));
    assert_eq!(prime_divisors(17), HashMap::from([(17u64, 1),]));
    assert_eq!(prime_divisors(8), HashMap::from([(2u64, 3),]));
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

fn combinate(n: u64, r: u64) -> bool {
    let n_fac = factorial(n)
        .iter()
        .map(|x| prime_divisors(*x))
        .collect::<Vec<_>>();
    let r_fac = factorial(r)
        .iter()
        .map(|x| prime_divisors(*x))
        .collect::<Vec<_>>();
    let n_r_fac = factorial(n - r)
        .iter()
        .map(|x| prime_divisors(*x))
        .collect::<Vec<_>>();

    let mut top = combine_hashmaps(n_fac);
    let bottom = combine_hashmaps(vec![combine_hashmaps(r_fac), combine_hashmaps(n_r_fac)]);

    let mut result: f64 = 1.0;

    for (k, mut v) in bottom {
        loop {
            let count = top.entry(k).or_insert(0);
            if v == 0 || *count == 0 {
                break;
            }

            *count -= 1;
            v -= 1;
        }

        if v > 0 {
            result = result / ((k * v) as f64);
        }
    }

    for (k, v) in top {
        if v == 0 {
            continue;
        }

        for _ in 1..=v {
            result *= k as f64;
            if result > 1_000_000 as f64 {
                break;
            }
        }
    }

    result >= 1_000_000 as f64
}

#[test]
fn test_combinate() {
    assert_eq!(combinate(5, 3), false);
    assert_eq!(combinate(23, 10), true);
}

fn pe053() -> u64 {
    let mut over_1_million = 0u64;
    for n in 1..=100 {
        for r in 1..=n {
            if combinate(n, r) {
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
