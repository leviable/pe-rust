// Euler's totient function, [sometimes called the phi function],
// is used to determine the number of positive numbers less
// than or equal to which are relatively prime to . For example,
// as , and , are all less than nine and relatively prime to nine, .
// The number is considered to be relatively prime to every positive number, so
// .
//
// Interestingly, , and it can be seen that is a permutation of
// .
//
// Find the value of , , for which is a permutation of and the
// ratio produces a minimum.
use spmc;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

pub fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}

#[test]
fn test_gcd() {
    // Simple greatest common divisor.
    assert_eq!(gcd(3, 5), 1);
    assert_eq!(gcd(14, 15), 1);

    // More complex greatest common divisor.
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}

fn relative_primes(n: u64) -> Vec<u64> {
    let mut coprimes: Vec<u64> = vec![1];

    if n == 1 {
        return coprimes;
    }

    for num in 2..n {
        if gcd(num, n) == 1 {
            coprimes.push(num)
        }
    }

    coprimes
}

#[test]
fn test_relative_primes() {
    assert_eq!(relative_primes(2), vec![1]);
    assert_eq!(relative_primes(3), vec![1, 2]);
    assert_eq!(relative_primes(4), vec![1, 3]);
    assert_eq!(relative_primes(5), vec![1, 2, 3, 4]);
    assert_eq!(relative_primes(6), vec![1, 5]);
    assert_eq!(relative_primes(7), vec![1, 2, 3, 4, 5, 6]);
    assert_eq!(relative_primes(8), vec![1, 3, 5, 7]);
    assert_eq!(relative_primes(9), vec![1, 2, 4, 5, 7, 8]);
    assert_eq!(relative_primes(10), vec![1, 3, 7, 9]);
}

fn is_perm(n1: u64, n2: u64) -> bool {
    let mut n1 = n1.to_string().chars().collect::<Vec<_>>();
    let mut n2 = n2.to_string().chars().collect::<Vec<_>>();

    n1.sort();
    n2.sort();

    n1 == n2
}

fn pe070() -> u64 {
    let (mut work_tx, work_rx) = spmc::channel();
    let (result_tx, result_rx): (Sender<(u64, f64)>, Receiver<(u64, f64)>) = mpsc::channel();

    let mut children = Vec::new();

    let mut min_fraction = 1_000_000_000f64;
    let mut min_n = 1;

    for n in (2..9_000_000u64).rev() {
        work_tx.send(n).unwrap();
    }

    for _ in 1..8 {
        let rx_clone = work_rx.clone();
        let tx_clone = result_tx.clone();
        let child = thread::spawn(move || loop {
            let n = rx_clone.recv().unwrap();

            let coprimes = relative_primes(n);
            let phi = coprimes.len() as u64;

            if !is_perm(n, phi) {
                continue;
            }

            let frac: f64 = n as f64 / phi as f64;

            tx_clone.send((n, frac)).unwrap();
        });

        children.push(child);
    }
    loop {
        let (result_n, result_frac) = result_rx.recv().unwrap();
        if result_frac < min_fraction {
            min_fraction = result_frac;
            min_n = result_n;
        }

        println!("Found one: {} -> {} -> {}", result_n, min_n, min_fraction);
    }

    0
}

fn main() {
    let start = std::time::Instant::now();
    let solution = pe070();
    println!("Solution: {solution}");
    println!("Elapsed time: {}", start.elapsed().as_secs_f64());
}
