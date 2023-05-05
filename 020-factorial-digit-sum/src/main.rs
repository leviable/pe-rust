// n! means n × (n − 1) × ... × 3 × 2 × 1

// For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
// and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.

// Find the sum of the digits in the number 100!
use std::time::Instant;

struct BigNum {
    num: Vec<usize>,
}

impl BigNum {
    fn new() -> Self {
        Self { num: vec![1] }
    }

    fn mul(&mut self, mul_num: usize) {
        let mut recip = 0;
        let mut new_num: Vec<usize> = vec![];
        for digit in &self.num {
            new_num.push((digit * mul_num + recip) % 10);
            recip = (digit * mul_num + recip) / 10;
        }

        while recip > 9 {
            new_num.push(recip % 10);
            recip /= 10;
        }
        if recip > 0 {
            new_num.push(recip);
        }

        self.num = new_num;
    }

    fn factorial(&mut self, fac_num: usize) {
        for num in 1..=fac_num {
            self.mul(num);
        }
    }

    fn sum(&self) -> usize {
        let mut num_sum = 0;
        for digit in &self.num {
            num_sum += digit;
        }

        num_sum
    }
}

#[test]
fn test_bignum_multiply() {
    let mut b = BigNum::new();
    b.mul(2);
    assert_eq!(b.num, vec![2]);
}

#[test]
fn test_bignum_factorial() {
    let mut b = BigNum::new();
    b.factorial(10);
    assert_eq!(b.num, vec![0, 0, 8, 8, 2, 6, 3]);
}

#[test]
fn test_bignum_sum() {
    let mut b = BigNum::new();
    b.factorial(10);
    assert_eq!(b.sum(), 27);
}

fn main() {
    let start = Instant::now();
    let mut num = BigNum::new();
    num.factorial(100);
    println!("Factorial Sum is {}", num.sum());
    println!("Time to complete: {}", start.elapsed().as_secs_f64());
}
