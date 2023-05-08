// A unit fraction contains 1 in the numerator. The decimal representation of the unit fractions with denominators 2 to 10 are given:

// 1/2	= 	0.5
// 1/3	= 	0.(3)
// 1/4	= 	0.25
// 1/5	= 	0.2
// 1/6	= 	0.1(6)
// 1/7	= 	0.(142857)
// 1/8	= 	0.125
// 1/9	= 	0.(1)
// 1/10	= 	0.1
// Where 0.1(6) means 0.166666..., and has a 1-digit recurring cycle. It can be seen that 1/7 has a 6-digit recurring cycle.

// Find the value of d < 1000 for which 1/d contains the longest recurring cycle in its decimal fraction part.

use std::time::Instant;

#[derive(Debug, PartialEq)]
struct Frac {
    num: usize,
    den: usize,
    digits: Vec<usize>,
    div_history: Vec<Frac>,
    repeating: bool,
}

impl Frac {
    fn new(num: usize, den: usize) -> Self {
        Self {
            num: num,
            den: den,
            digits: vec![],
            div_history: vec![],
            repeating: false,
        }
    }
}

fn div(mut frac: Frac) -> Frac {
    for previous in &frac.div_history {
        if previous.num == frac.num && previous.den == frac.den {
            frac.repeating = true;
            return frac;
        }
    }

    frac.div_history.push(Frac::new(frac.num, frac.den));

    if frac.num % frac.den == 0 {
        frac.digits.push(frac.num / frac.den);
        return frac;
    }

    if !(frac.num >= frac.den) {
        frac.num *= 10;
        frac.digits.push(0);

        return div(frac);
    }

    frac.digits.push(frac.num / frac.den);
    frac.num = frac.num % frac.den * 10;

    div(frac)
}

#[test]
fn test_div() {
    // Rational numbers
    let f = div(Frac::new(1, 2));
    assert_eq!(f.digits, vec![0, 5]);
    assert_eq!(f.repeating, false);
    let f = div(Frac::new(1, 4));
    assert_eq!(f.digits, vec![0, 2, 5]);
    assert_eq!(f.repeating, false);
    let f = div(Frac::new(1, 5));
    assert_eq!(f.digits, vec![0, 2]);
    assert_eq!(f.repeating, false);
    let f = div(Frac::new(1, 8));
    assert_eq!(f.digits, vec![0, 1, 2, 5]);
    assert_eq!(f.repeating, false);
    let f = div(Frac::new(1, 10));
    assert_eq!(f.digits, vec![0, 1]);
    assert_eq!(f.repeating, false);

    // Single digit repeating
    let f = div(Frac::new(1, 3));
    assert_eq!(f.digits, vec![0, 3]);
    assert_eq!(f.repeating, true);
    let f = div(Frac::new(1, 6));
    assert_eq!(f.digits, vec![0, 1, 6]);
    assert_eq!(f.repeating, true);
    let f = div(Frac::new(1, 9));
    assert_eq!(f.digits, vec![0, 1]);
    assert_eq!(f.repeating, true);

    // Multi digit repeating
    let f = div(Frac::new(1, 7));
    assert_eq!(f.digits, vec![0, 1, 4, 2, 8, 5, 7]);
    assert_eq!(f.repeating, true);
}

fn main() {
    let start = Instant::now();
    let mut longest_repeating_idx = 0;
    let mut longest_repeating_frac: Frac = Frac::new(1, 2);
    for den in 2..1000 {
        let f = div(Frac::new(1, den));
        if !f.repeating {
            continue;
        }

        if f.digits.len() > longest_repeating_frac.digits.len() {
            longest_repeating_idx = den;
            longest_repeating_frac = f;
            println!(
                "New longest: {} with len: {}",
                longest_repeating_idx,
                longest_repeating_frac.digits.len()
            );
        }
    }
    println!("Elapsed time: {}", start.elapsed().as_secs_f64());
}
