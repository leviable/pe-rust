// An irrational decimal fraction is created by concatenating the positive integers:

// 0.123456789101112131415161718192021...
//              ^
// It can be seen that the 12th digit of the fractional part is 1.

// If dn represents the nth digit of the fractional part, find the value of the
// following expression.

// d1 × d10 × d100 × d1000 × d10000 × d100000 × d1000000

use std::time::Instant;

fn pe040() -> u32 {
    let mut product: u32 = 1;
    let mut d_frac: Vec<char> = vec!['0'];
    for d in 1..1_000_000 {
        for c in d.to_string().chars() {
            d_frac.push(c);
        }

        match d {
            1 | 10 | 100 | 1_000 | 10_000 | 100_000 | 1_000_000 => {
                product *= d_frac[d].to_digit(10).unwrap()
            }
            _ => (),
        };
    }

    product
}

fn main() {
    let start = Instant::now();
    let solution = pe040();
    println!("Solution is: {solution}");
    println!("Time elapsed: {}", start.elapsed().as_secs_f64());
}
