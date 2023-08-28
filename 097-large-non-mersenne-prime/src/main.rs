// The first known prime found to exceed one million digits was discovered in 1999,
// and is a Mersenne prime of the form p**6972593 - 1; it contains exactly 2_098_960 digits.
// Subsequently other Mersenne primes, of the form 2**P - 1, have been found which contain
// more digits.
//
// However, in 2004 there was found a massive non-Mersenne prime which contains
// 2_357_207 digits: 28433 X 2**7830457 + 1.
//
// Find the last ten digits of this prime number.

use std::cmp::Ordering;

#[derive(Default)]
struct MPrime {
    base: u64,
    val: String,
}

impl MPrime {
    fn new(base: u64) -> Self {
        Self {
            base,
            val: base.to_string(),
        }
    }

    fn pow(&mut self, to: u64) {
        for _ in 1..to {
            let mut new_val = self.val.parse::<u64>().unwrap();
            new_val *= self.base;
            let new_val = new_val.to_string();

            let slice_len = match new_val.len().cmp(&10) {
                Ordering::Equal | Ordering::Greater => 10,
                Ordering::Less => new_val.len(),
            };

            self.val = new_val[(new_val.len() - slice_len)..new_val.len()].to_string();
        }
    }
}

#[test]
fn test_m_prime() {
    let mut m = MPrime::new(2);
    assert_eq!(m.val, 2.to_string());

    m.pow(2);
    assert_eq!(m.val, 4.to_string());

    let mut m = MPrime::new(2);
    m.pow(3);
    assert_eq!(m.val, 8.to_string());
}

fn pe097() -> u64 {
    let mut num = MPrime::new(2);
    num.pow(7830457);
    (num.val.parse::<u64>().unwrap() * 28433) + 1
}

// 5583391273
fn main() {
    let start = std::time::Instant::now();
    let solution = pe097();
    println!("Solution: {solution}");
    println!("Time elapsed: {}", start.elapsed().as_secs_f64());
}
