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

fn factors(n: usize) -> Vec<usize> {
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

fn relative_primes(n: usize) -> Vec<usize> {
    let mut coprimes: Vec<usize> = vec![1];

    if n == 1 {
        return coprimes;
    }

    let n_factors = factors(n);

    'outer: for idx in 2..n {
        let idx_factors = factors(idx);

        if n % idx == 0 {
            continue;
        }

        let mut combined = vec![];
        combined.extend(n_factors.clone());
        combined.extend(idx_factors.clone());
        combined.dedup();

        for c in combined {
            if c == 1 {
                continue;
            }
            if n_factors.contains(&c) && idx_factors.contains(&c) {
                continue 'outer;
            }
        }
        coprimes.push(idx);
    }

    coprimes.dedup();
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

fn phi(n: usize) -> usize {
    relative_primes(n).len()
}

#[test]
fn test_phi() {
    assert_eq!(phi(2), 1);
    assert_eq!(phi(3), 2);
    assert_eq!(phi(4), 2);
    assert_eq!(phi(5), 4);
    assert_eq!(phi(6), 2);
    assert_eq!(phi(7), 6);
    assert_eq!(phi(8), 4);
    assert_eq!(phi(9), 6);
    assert_eq!(phi(10), 4);
}

fn pe069(limit: usize) -> usize {
    let mut max_phi = 0;
    let mut max_n_over_phi = 0.0;

    for n in (2..=limit).into_iter() {
        // eprintln!("Working on: {n}");
        let r_primes = relative_primes(n);
        let phi = r_primes.len();
        let n_over_phi = n as f64 / phi as f64;

        if n_over_phi > max_n_over_phi {
            max_phi = n;
            max_n_over_phi = n_over_phi;
            eprintln!("New Max: {max_phi} -> {:?}", r_primes);
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
