// It was proposed by Christian Goldbach that every odd composite number can be
// written as the sum of a prime and twice a square.
//
//   9 =  7 + 2 * 1^2
//  15 =  7 + 2 * 2^2
//  21 =  3 + 2 * 3^2
//  25 =  7 + 2 * 3^2
//  27 = 19 + 2 * 2^2
//  33 = 31 + 2 * 1^2
//
//
// It turns out that the conjecture was false.
//
// What is the smallest odd composite that cannot be written as the sum of a
// prime and twice a square?

use primes::*;
use std::cmp::Ordering;
use std::time::Instant;

fn check_composite(want: u32, prime: u32, power: u32) -> Ordering {
    match prime + (2 * power.pow(2)) {
        x if x < want => Ordering::Less,
        x if x == want => Ordering::Equal,
        x if x > want => Ordering::Greater,
        _ => unreachable!("This can't happen"),
    }
}

#[test]
fn test_check_composite() {
    assert_eq!(check_composite(9, 3, 1), Ordering::Less);
    assert_eq!(check_composite(9, 3, 2), Ordering::Greater);
    assert_eq!(check_composite(9, 5, 1), Ordering::Less);
    assert_eq!(check_composite(9, 5, 2), Ordering::Greater);
    assert_eq!(check_composite(9, 7, 1), Ordering::Equal);
}

fn main() {
    let start = Instant::now();
    'outer: for num in (3..)
        .filter(|x| x % 2 == 1)
        .filter(|y| !primes::is_prime(*y))
    {
        let mut pset = Sieve::new();
        'inner: for p in pset.iter() {
            if p > num {
                println!("11111111111111111111111111111111111111");
                println!("Found it: {num}");
                println!("11111111111111111111111111111111111111");
                break 'outer;
            }
            for pow in 1.. {
                match check_composite(num as u32, p as u32, pow as u32) {
                    Ordering::Less => continue,
                    Ordering::Equal => continue 'outer,
                    Ordering::Greater => continue 'inner,
                };
            }
        }
    }
    println!("Time elapsed: {}", start.elapsed().as_secs_f64());
}
