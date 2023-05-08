// Euler discovered the remarkable quadratic formula:

use primes::is_prime;
use std::time::Instant;

fn quad(n: i64, a: i64, b: i64) -> i64 {
    return n.pow(2) + (a * n) + b;
}

#[test]
fn test_quad() {
    assert_eq!(quad(0, 0, 41), 41);
}

fn prime_count(a: i64, b: i64) -> i64 {
    let mut count = 0;
    for n in 0..b {
        let q = quad(n, a, b);
        if !is_prime(q.abs() as u64) {
            break;
        } else {
            count += 1;
        }
    }
    count
}

#[test]
fn test_prime_count() {
    assert_eq!(prime_count(1, 41), 40);
    assert_eq!(prime_count(-79, 1601), 80);
}

fn main() {
    let start = Instant::now();
    let mut most_prime_coef = (0, 0);
    let mut most_prime_count = 0;

    for a in -1000..1000 {
        for b in -1000..1000 {
            let count = prime_count(a, b);
            if count > most_prime_count {
                println!("Found a new one: {a} {b} {count}");
                most_prime_count = count;
                most_prime_coef = (a, b);
            }
        }
    }

    println!(
        "Most primes of {} for {:?}",
        most_prime_count, most_prime_coef
    );

    println!("Elapsed time: {}", start.elapsed().as_secs_f64());
}
