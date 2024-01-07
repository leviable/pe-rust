// Both 169 and 961 are the square of a prime. 169 is the reverse of 961 .
//
// We call a number a reversible prime square if:
//
// * It is not a palindrome, and
// * It is the square of a prime, and
// * Its reverse is also the square of a prime.
//
//  169 and 961 are not palindromes, so both are reversible prime squares.
//
// Find the sum of the first 50 reversible prime squares.

use primes::{is_prime, PrimeSet, Sieve};

fn is_palindrome(num: u64) -> bool {
    num == num
        .to_string()
        .chars()
        .rev()
        .collect::<String>()
        .parse()
        .unwrap()
}

#[test]
fn test_is_palindrome() {
    assert!(!is_palindrome(123));
    assert!(is_palindrome(121));
}

fn is_rev_prime(prime_square: u64) -> bool {
    let rev = prime_square
        .to_string()
        .chars()
        .rev()
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    let rev_sqrt = (rev as f64).sqrt();

    if rev_sqrt != (rev_sqrt as u64) as f64 {
        return false;
    }

    is_prime(rev_sqrt as u64)
}

#[test]
fn test_is_rev_prime() {
    assert!(is_rev_prime(169));
    assert!(is_rev_prime(961));
    assert!(!is_rev_prime(841));
    assert!(!is_rev_prime(25));
    assert!(!is_rev_prime(289));
}

fn pe808() -> u64 {
    let mut rev_primes = vec![];

    for prime in Sieve::new().iter() {
        let prime_square = prime.pow(2);
        if is_rev_prime(prime_square) && !is_palindrome(prime_square) {
            rev_primes.push(prime_square);
        }

        if rev_primes.len() >= 50 {
            break;
        }
    }

    rev_primes.iter().sum()
}

fn main() {
    let start = std::time::Instant::now();
    let solution = pe808();
    println!("Solution: {solution}");
    println!("Time elapsed: {}", start.elapsed().as_secs_f64());
}
