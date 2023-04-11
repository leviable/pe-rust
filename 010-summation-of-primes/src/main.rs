// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.

// Find the sum of all the primes below two million.

fn main() {
    let primes = primes_under(2_000_000);
    // println!("Primes are: {:?}", &primes[primes.len() - 1]);
    let mut total: u64 = 0;
    for val in primes {
        total += val as u64;
    }
    println!("Prime sum: {}", total);
}

fn primes_under(limit: u32) -> Vec<u32> {
    let mut sieve = vec![true; limit as usize];
    let mut primes = Vec::new();
    primes.push(2);
    for index in (3..limit - 1).step_by(2) {
        if sieve[index as usize] {
            primes.push(index);
            for x in (index..limit - 1).step_by(index as usize) {
                sieve[x as usize] = false;
            }
        }
    }
    primes
}

#[test]
fn test_primes_under() {
    assert_eq!(primes_under(10), vec![2, 3, 5, 7]);
}
