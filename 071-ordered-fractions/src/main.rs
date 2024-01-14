// Consider the fraction, , where and are positive integers. If and ,
// it is called a reduced proper fraction.
//
// If we list the set of reduced proper fractions for
//  in ascending order of size, we get:
//
// It can be seen that is the fraction immediately to the left of .
//
// By listing the set of reduced proper fractions for
//  in ascending order of size, find the numerator of the fraction immediately to the left of
//
// .
use std::collections::HashSet;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
struct Fraction {
    num: usize,
    den: usize,
}

impl Fraction {
    fn new(num: usize, den: usize) -> Self {
        Self { num, den }
    }

    fn reduce(&mut self) {
        'reduce_loop: loop {
            for num in 2..=self.num {
                if self.num % num == 0 && self.den % num == 0 {
                    self.num /= num;
                    self.den /= num;
                    continue 'reduce_loop;
                }
            }
            break;
        }
    }
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.num, self.den)
    }
}

fn pe071(limit: usize) -> Fraction {
    let mut hs: HashSet<Fraction> = HashSet::new();
    for den in 2..=limit {
        if den % 1000 == 0 {
            eprintln!("Working on: {den}");
        }
        for num in (den * 42_857 / 100_000)..=(den * 3 / 7) {
            let mut frac = Fraction::new(num, den);
            frac.reduce();
            hs.insert(frac);
        }
    }

    let mut sorted_fracs: Vec<Fraction> = hs.into_iter().collect();
    // floats.sort_by(|a, b| a.partial_cmp(b).unwrap());
    // sorted_fracs.sort_by_key(|x| x.num / x.den);
    sorted_fracs.sort_by(|a, b| {
        (a.num as f64 / a.den as f64)
            .partial_cmp(&(b.num as f64 / b.den as f64))
            .unwrap()
    });

    for x in sorted_fracs.clone() {
        eprintln!("{}", x);
    }
    match sorted_fracs
        .clone()
        .iter()
        .position(|x| x == &Fraction { num: 3, den: 7 })
    {
        Some(x) => sorted_fracs[x - 1].clone(),
        None => panic!("Didn't find it"),
    }
}

#[test]
fn test_pe071() {
    assert_eq!(pe071(8), Fraction { num: 2, den: 5 });
}

fn main() {
    let start = std::time::Instant::now();
    let solution = pe071(1_000_000usize);
    println!("Solution: {}", solution);
    println!("Time elapsed: {}", start.elapsed().as_secs_f64());
}
