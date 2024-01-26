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

fn pe069(limit: u64) -> u64 {
    let mut max_phi = 0;
    let mut max_n_over_phi = 0.0;

    for n in (0..=limit).into_iter().step_by(10) {
        if n % 1_000 == 0 {
            eprintln!("Working on {n} -> Max is {max_phi}");
        }
        let r_primes = relative_primes(n);
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
    let solution = pe069(1_000_000);
    println!("Solution: {solution}");
    println!("Time elapsed: {}", start.elapsed().as_secs_f64());
}
