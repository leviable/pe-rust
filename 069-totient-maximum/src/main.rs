// Euler's totient function, [sometimes called the phi function], is defined as
// the number of positive integers not exceeding which are relatively prime to .
// For example, as , and , are all less than or equal to nine and relatively prime to nine,
// .
//
// Relatively Prime
// 2	1	1	2
// 3	1,2	2	1.5
// 4	1,3	2	2
// 5	1,2,3,4	4	1.25
// 6	1,5	2	3
// 7	1,2,3,4,5,6	6	1.1666...
// 8	1,3,5,7	4	2
// 9	1,2,4,5,7,8	6	1.5
// 10	1,3,7,9	4	2.5
// It can be seen that produces a maximum for .
//
// Find the value of for which is a maximum.

use primes::{is_prime, PrimeSet, Sieve};

fn factors(n: u64) -> Vec<u64> {
    let mut factors = vec![1];

    if n == 1 {
        return factors;
    }

    for x in 2..=(n / 2) {
        if n % x == 0 {
            factors.push(x);
        }
    }

    factors
}

#[test]
fn test_factors() {
    assert_eq!(factors(1), vec![1]);
    assert_eq!(factors(2), vec![1]);
    assert_eq!(factors(3), vec![1]);
    assert_eq!(factors(4), vec![1, 2]);
    assert_eq!(factors(5), vec![1]);
    assert_eq!(factors(6), vec![1, 2, 3]);
    assert_eq!(factors(7), vec![1]);
    assert_eq!(factors(8), vec![1, 2, 4]);
    assert_eq!(factors(9), vec![1, 3]);
    assert_eq!(factors(10), vec![1, 2, 5]);
}

fn relative_primes(primes_list: &Vec<u64>, n: u64) -> Vec<u64> {
    let mut coprimes: Vec<u64> = vec![1];

    if n == 1 {
        return coprimes;
    }

    for prime in primes_list.iter().take_while(|x| x < &&n) {
        if n % prime == 0 {
            continue;
        }
        coprimes.push(*prime)
    }

    coprimes
}

#[test]
fn test_relative_primes() {
    let prime_list = Sieve::new().iter().take(20).collect::<Vec<_>>();
    assert_eq!(relative_primes(&prime_list, 2), vec![1]);
    assert_eq!(relative_primes(&prime_list, 3), vec![1, 2]);
    assert_eq!(relative_primes(&prime_list, 4), vec![1, 3]);
    // assert_eq!(relative_primes(&prime_list, 5), vec![1, 2, 3, 4]);
    assert_eq!(relative_primes(&prime_list, 6), vec![1, 5]);
    // assert_eq!(relative_primes(&prime_list, 7), vec![1, 2, 3, 4, 5, 6]);
    assert_eq!(relative_primes(&prime_list, 8), vec![1, 3, 5, 7]);
    // assert_eq!(relative_primes(&prime_list, 9), vec![1, 2, 4, 5, 7, 8]);
    // assert_eq!(relative_primes(&prime_list, 10), vec![1, 3, 7, 9]);
}

// fn phi(n: usize) -> usize {
//     relative_primes(n).len()
// }
//
// #[test]
// fn test_phi() {
//     assert_eq!(phi(2), 1);
//     assert_eq!(phi(3), 2);
//     assert_eq!(phi(4), 2);
//     assert_eq!(phi(5), 4);
//     assert_eq!(phi(6), 2);
//     assert_eq!(phi(7), 6);
//     assert_eq!(phi(8), 4);
//     assert_eq!(phi(9), 6);
//     assert_eq!(phi(10), 4);
// }

fn pe069(limit: u64) -> u64 {
    let prime_list = Sieve::new()
        .iter()
        .take_while(|x| x <= &limit)
        .collect::<Vec<_>>();

    let mut max_phi = 0;
    let mut max_n_over_phi = 0.0;

    for n in (2..=limit).into_iter() {
        let r_primes = relative_primes(&prime_list, n);
        let phi = r_primes.len();
        let n_over_phi = n as f64 / phi as f64;

        if n_over_phi > max_n_over_phi {
            max_phi = n;
            max_n_over_phi = n_over_phi;
            eprintln!("New Max: {max_phi}");
        }
    }

    max_phi
}

#[test]
fn test_pe069() {
    assert_eq!(pe069(10), 6);
}

fn main() {
    let start = std::time::Instant::now();
    let solution = pe069(1_000);
    println!("Solution: {solution}");
    println!("Time elapsed: {}", start.elapsed().as_secs_f64());
}
