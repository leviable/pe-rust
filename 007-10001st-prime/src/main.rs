// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.

// What is the 10 001st prime number?

fn main() {
    println!("The 10_001st prime is: {}", xth_prime(10_001));
}

fn xth_prime(index: usize) -> usize {
    prime_sieve(index)[index - 1]
}

#[test]
fn get_xth_prime() {
    assert_eq!(xth_prime(6), 13);
}

fn prime_sieve(limit: usize) -> Vec<usize> {
    let upper_bound = limit * 1000;
    let mut sieve = vec![true; upper_bound];
    let mut primes = Vec::new();

    for num in 2..upper_bound {
        if sieve[num] == false {
            continue;
        }
        println!("Found prime: {}", num);
        primes.push(num);
        for index in (num..upper_bound).step_by(num) {
            sieve[index] = false;
        }

        if primes.len() >= limit {
            break;
        }
    }
    primes
}

#[test]
fn test_prime_sieve() {
    assert_eq!(prime_sieve(1), vec![2]);
    assert_eq!(prime_sieve(2), vec![2, 3]);
    assert_eq!(prime_sieve(6), vec![2, 3, 5, 7, 11, 13]);
}
