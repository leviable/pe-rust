// The prime factors of 13195 are 5, 7, 13 and 29.

// What is the largest prime factor of the number 600851475143 ?

fn main() {
    let largest_prime = largest_prime_factor(600_851_475_143);
    println!("Largest prime factor is {}", largest_prime);
}

fn largest_prime_factor(num: u64) -> u64 {
    let primes = prime_factors(num);
    primes[primes.len() - 1]
}

#[test]
fn test_largest_prime_factor() {
    assert_eq!(largest_prime_factor(4), 2);
    assert_eq!(largest_prime_factor(13195), 29);
}

fn prime_factors(num: u64) -> Vec<u64> {
    let mut primes = Vec::new();
    let upper_limit = (num as f64).sqrt();
    for div in 2..upper_limit as u64 + 1 {
        if num % div == 0 && is_prime(div) {
            primes.push(div);
        }
    }
    println!("This is {:?}", primes);
    primes
}

#[test]
fn test_prime_factors() {
    assert_eq!(prime_factors(16), vec![2]);
    assert_eq!(prime_factors(13195), vec![5, 7, 13, 29]);
}

fn is_prime(num: u64) -> bool {
    if num == 2 || num == 3 || num == 5 {
        return true;
    }
    if num % 2 == 0 {
        return false;
    }

    let upper_limit = (num as f64).sqrt();
    for div in 3..upper_limit as u64 + 1 {
        if num % div == 0 {
            return false;
        }
    }

    true
}

#[test]
fn test_is_prime() {
    assert_eq!(is_prime(3), true);
    assert_eq!(is_prime(4), false);
    assert_eq!(is_prime(5), true);
    assert_eq!(is_prime(6), false);
    assert_eq!(is_prime(9), false);
    assert_eq!(is_prime(111), false);
}
