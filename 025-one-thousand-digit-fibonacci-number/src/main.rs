// The Fibonacci sequence is defined by the recurrence relation:

// Fn = Fn−1 + Fn−2, where F1 = 1 and F2 = 1.
// Hence the first 12 terms will be:

// F1 = 1
// F2 = 1
// F3 = 2
// F4 = 3
// F5 = 5
// F6 = 8
// F7 = 13
// F8 = 21
// F9 = 34
// F10 = 55
// F11 = 89
// F12 = 144
// The 12th term, F12, is the first term to contain three digits.

// What is the index of the first term in the Fibonacci sequence to contain 1000 digits?

use itertools::{EitherOrBoth::*, Itertools};
use std::time::Instant;

#[derive(PartialEq, Debug)]
struct BigNum {
    digits: Vec<u32>,
}

impl BigNum {
    fn new(num: usize) -> Self {
        let binding = num.to_string();
        let digits: Vec<u32> = binding
            .chars()
            .map(|x| x.to_digit(10).unwrap())
            .collect::<Vec<u32>>()
            .into_iter()
            .rev()
            .collect();
        Self { digits: digits }
    }

    fn len(&self) -> usize {
        self.digits.len()
    }
}

impl Clone for BigNum {
    fn clone(&self) -> Self {
        Self {
            digits: self.digits.clone(),
        }
    }
}

fn add(num1: &BigNum, num2: &BigNum) -> BigNum {
    let mut recip = 0;

    let mut new_digits: Vec<u32> = vec![];

    for pair in num1.digits.iter().zip_longest(num2.digits.clone()) {
        let (l, r) = match pair {
            Both(l, r) => (l, r),
            Left(l) => (l, 0),
            Right(r) => (&0, r),
        };
        let sum = l + r + recip;
        recip = sum / 10;
        new_digits.push(sum % 10);
    }

    while recip > 0 {
        new_digits.push(recip % 10);
        recip /= 10;
    }

    BigNum { digits: new_digits }
}

// impl<'a, 'b> Add<&'b BigNum> for &'a BigNum {
//     type Output = Self;

//     fn add(self, other: &'b Self) -> Self {
//         let mut recip = 0;

//         let mut new_digits: Vec<u32> = vec![];
//         for (l, r) in self.digits.iter().zip(other.digits) {
//             let sum = l + r + recip;
//             recip = sum / 10;
//             new_digits.push(sum % 10);
//         }

//         Self { digits: new_digits }
//     }
// }

#[test]
fn test_bignum_new() {
    let num = BigNum::new(123456);
    assert_eq!(num.digits, vec![6, 5, 4, 3, 2, 1]);
}

#[test]
fn test_bignum_len() {
    let num = BigNum::new(123456);
    assert_eq!(num.len(), 6);
}

#[derive(Debug, PartialEq)]
struct Fib {
    nums: Vec<BigNum>,
}

impl Fib {
    fn new(num1: usize, num2: usize) -> Self {
        Self {
            nums: vec![BigNum::new(num1), BigNum::new(num2)],
        }
    }

    fn next(&mut self) {
        let new_num = add(&self.nums[0], &self.nums[1]);
        let new_num0 = &self.nums[1].clone();
        // self.nums[0] = self.nums[1];
        self.nums[1] = new_num;
        self.nums[0] = new_num0.clone();
    }

    fn len(&self) -> usize {
        self.nums[1].len()
    }
}

#[test]
fn test_fib_next() {
    let mut f = Fib::new(1, 1);
    assert_eq!(f.nums[0], BigNum::new(1));
    assert_eq!(f.nums[1], BigNum::new(1));
    f.next();
    assert_eq!(f.nums[0], BigNum::new(1));
    assert_eq!(f.nums[1], BigNum::new(2));
    f.next();
    assert_eq!(f.nums[0], BigNum::new(2));
    assert_eq!(f.nums[1], BigNum::new(3));
    f.next();
    assert_eq!(f.nums[0], BigNum::new(3));
    assert_eq!(f.nums[1], BigNum::new(5));
    f.next();
    assert_eq!(f.nums[0], BigNum::new(5));
    assert_eq!(f.nums[1], BigNum::new(8));
    f.next();
    assert_eq!(f.nums[0], BigNum::new(8));
    assert_eq!(f.nums[1], BigNum::new(13));
    f.next();
    assert_eq!(f.nums[0], BigNum::new(13));
    assert_eq!(f.nums[1], BigNum::new(21));
    f.next();
    assert_eq!(f.nums[0], BigNum::new(21));
    assert_eq!(f.nums[1], BigNum::new(34));
    f.next();
    assert_eq!(f.nums[0], BigNum::new(34));
    assert_eq!(f.nums[1], BigNum::new(55));
    f.next();
    assert_eq!(f.nums[0], BigNum::new(55));
    assert_eq!(f.nums[1], BigNum::new(89));
    f.next();
    assert_eq!(f.nums[0], BigNum::new(89));
    assert_eq!(f.nums[1], BigNum::new(144));
}

fn main() {
    let start = Instant::now();

    let mut f = Fib::new(1, 1);
    let mut count = 2;
    loop {
        count += 1;
        f.next();

        if f.len() >= 1000 {
            break;
        }
    }

    println!("Index for first 1000 len term is {count}");
    println!("Elapsed time: {}", start.elapsed().as_secs_f64());
}
