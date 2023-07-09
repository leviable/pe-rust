// NOTE: can't really convert this one to comments because of formatting
//
// It is possible to show that the square root of two can be expressed as
// an infinite continued fraction.
//
//
//
//
//
// By expanding this for the first four iterations, we get:
//
// The next three expansions are
//
// ,
//
// , and
//
// , but the eighth expansion,
//
// , is the first example where the number of digits in the numerator exceeds the number of digits in the denominator.
//
// In the first one-thousand expansions, how many fractions contain a numerator with more digits than the denominator?

use num_bigint::{BigUint, ToBigUint};
use std::time::Instant;

#[derive(Debug, Eq, PartialEq)]
struct Fraction {
    nume: BigUint,
    deno: BigUint,
}

impl Fraction {
    fn new(nume: u128, deno: u128) -> Self {
        Self {
            nume: nume.to_biguint().unwrap(),
            deno: deno.to_biguint().unwrap(),
        }
    }

    fn add(&mut self, val: u128) {
        self.nume = val.to_biguint().unwrap() * &self.deno + &self.nume;
    }

    fn flip(&mut self) {
        (self.nume, self.deno) = (self.deno.clone(), self.nume.clone());
    }

    fn bigger_nume(&mut self) -> bool {
        self.nume.to_string().len() > self.deno.to_string().len()
    }
}

#[test]
fn test_fraction_add() {
    let frac = &mut Fraction::new(1, 2);
    frac.add(1);
    assert_eq!(frac, &Fraction::new(3, 2));
}

#[test]
fn test_fraction_flip() {
    let frac = &mut Fraction::new(1, 2);
    frac.flip();
    assert_eq!(frac, &Fraction::new(2, 1));
}

#[test]
fn test_fractio_bigger_nume() {
    let frac = &mut Fraction::new(1, 2);
    assert!(!frac.bigger_nume());
    let frac = &mut Fraction::new(1393, 985);
    assert!(frac.bigger_nume());
}

fn recurse(num: u128, depth: u128) -> Fraction {
    if depth == 1 {
        let mut f = Fraction::new(1, 2);
        f.add(num);
        return f;
    } else {
        let mut f = recurse(2, depth - 1);
        f.flip();
        f.add(num);
        return f;
    }
}

#[test]
fn test_recurse() {
    assert_eq!(recurse(1, 1), Fraction::new(3, 2));
    assert_eq!(recurse(1, 2), Fraction::new(7, 5));
    assert_eq!(recurse(1, 3), Fraction::new(17, 12));
    assert_eq!(recurse(1, 4), Fraction::new(41, 29));
}

fn pe057() -> u128 {
    let mut solution = 0;
    for depth in 1..=1_000 {
        let mut f = recurse(1, depth);
        if f.bigger_nume() {
            solution += 1;
        }
    }

    solution
}

fn main() {
    let start = Instant::now();
    let solution = pe057();
    println!("Solution: {solution}");
    println!("Time elapsed: {}", start.elapsed().as_secs_f64());
}
