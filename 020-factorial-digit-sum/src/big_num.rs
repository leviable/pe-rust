// n! means n × (n − 1) × ... × 3 × 2 × 1

// For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
// and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.

// Find the sum of the digits in the number 100!
#[derive(Default, Debug)]
pub struct BigNum {
    pub num: Vec<usize>,
}

impl BigNum {
    pub fn new() -> Self {
        Self { num: vec![1] }
    }

    pub fn mul(&mut self, mul_num: usize) {
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

    pub fn factorial(&mut self, fac_num: usize) {
        for num in 1..=fac_num {
            self.mul(num);
        }
    }

    pub fn sum(&self) -> usize {
        let mut num_sum = 0;
        for digit in &self.num {
            num_sum += digit;
        }

        num_sum
    }

    pub fn value(&self) -> usize {
        self.num
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join("")
            .parse::<usize>()
            .unwrap()
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
