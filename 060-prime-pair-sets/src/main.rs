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

fn perms_are_prime(primes: &Vec<u64>) -> bool {
    primes
        .iter()
        .permutations(2)
        .map(|x| {
            x.iter()
                .map(|y| y.to_string())
                .collect::<String>()
                .parse()
                .unwrap()
        })
        .collect::<Vec<u64>>()
        .iter()
        .all(|z| is_prime(*z))
}

#[test]
fn test_perms_are_prime() {
    assert!(perms_are_prime(&vec![3, 7, 109, 673]));
    assert!(!perms_are_prime(&vec![2, 7, 109, 673]));
}

fn pe060() -> u64 {
    let mut p5_idx = 109;
    let mut p4_idx = 4;
    let mut p3_idx = 3;
    let mut p2_idx = 2;
    let mut p1_idx = 1;
    let sieve = Sieve::new().iter().take(1_000_000).collect::<Vec<_>>();

    let mut count = 0;

    'mainloop: loop {
        p5_idx += 1;
        count += 1;

        for p4_idx in p4_idx..p5_idx {
            for p3_idx in p3_idx..p4_idx {
                for p2_idx in p2_idx..p3_idx {
                    for p1_idx in p1_idx..p2_idx {
                        if count % 10 == 0 {
                            println!(
                                "{} {} {} {} {}",
                                sieve[p1_idx],
                                sieve[p2_idx],
                                sieve[p3_idx],
                                sieve[p4_idx],
                                sieve[p5_idx],
                            );
                        }
                        let candidates = vec![
                            sieve[p1_idx],
                            sieve[p2_idx],
                            sieve[p3_idx],
                            sieve[p4_idx],
                            sieve[p5_idx],
                        ];
                        if perms_are_prime(&candidates) {
                            println!("Found one: {:?}", candidates);

                            break 'mainloop;
                        }
                    }
                }
            }
        }
    }

    // 'loop1: for (p1_idx, p1) in Sieve::new().iter().skip(1).enumerate() {
    //     for (p2_idx, p2) in Sieve::new().iter().skip(p1_idx + 2).enumerate() {
    //         for (p3_idx, p3) in Sieve::new().iter().skip(p2_idx + 3).enumerate() {
    //             for (p4_idx, p4) in Sieve::new().iter().skip(p3_idx + 4).enumerate() {
    //                 for p5 in Sieve::new().iter().skip(p4_idx + 5) {
    //                     let candidates = vec![p1, p2, p3, p4, p5];
    //                     println!("Checking: {:?}", candidates);
    //                     if !perms_are_prime(&candidates) {
    //                         continue;
    //                     }
    //
    //                     let sum = candidates.iter().sum();
    //
    //                     if sum < lowest {
    //                         println!("Found one: {sum}");
    //                         lowest = sum;
    //                         break;
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
    0
}

fn main() {
    let start = std::time::Instant::now();
    let solution = pe060();
    println!("Solution: {solution}");
    println!("Time elapsed: {}", start.elapsed().as_secs_f64());
}
