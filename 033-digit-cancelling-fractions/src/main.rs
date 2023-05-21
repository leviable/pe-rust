// The fraction 49/98 is a curious fraction, as an inexperienced mathematician in attempting to simplify it may incorrectly believe that 49/98 = 4/8, which is correct, is obtained by cancelling the 9s.

// We shall consider fractions like, 30/50 = 3/5, to be trivial examples.

// There are exactly four non-trivial examples of this type of fraction, less than one in value, and containing two digits in the numerator and denominator.

// If the product of these four fractions is given in its lowest common terms, find the value of the denominator.

// use float_cmp::approx_eq;
use std::time::Instant;

fn get_variations(num: usize, den: usize) -> Vec<f64> {
    let mut variations: Vec<f64> = vec![];

    let num_str = num.to_string();
    let den_str = den.to_string();
    for (idx, num_digit) in num_str.chars().enumerate() {
        let den_digit = den_str.chars().collect::<Vec<_>>()[(idx + 1) % 2];
        if den_digit == num_digit {
            let new_num = num_str.chars().collect::<Vec<_>>()[(1 + idx) % 2]
                .to_digit(10)
                .unwrap() as f64;
            let new_den = den_str.chars().collect::<Vec<_>>()[idx]
                .to_digit(10)
                .unwrap() as f64;

            variations.push(new_num / new_den);
        }
    }
    variations
}

fn solve_033() -> (usize, usize) {
    let mut solutions: Vec<(usize, usize)> = vec![];
    for den in 10..=99 {
        for num in 10..den {
            // Ignore trivial case of multiples of 10
            if den % 10 == 0 && num % 10 == 0 {
                continue;
            }
            let actual_num: f64 = num as f64 / den as f64;

            for new_num in get_variations(num, den) {
                let difference = (new_num - actual_num).abs();
                if difference < 0.0001 {
                    println!("Found one {}/{}, {}", num, den, difference);
                    solutions.push((num, den));
                }
            }
        }
    }
    let big_num: usize = solutions.iter().map(|x| x.0).product();
    let big_den: usize = solutions.iter().map(|x| x.1).product();

    (big_num, big_den)
}

fn main() {
    let start = Instant::now();
    let solution = solve_033();
    println!("Solution to 033 is: {:?}", solution);
    println!("Elapsed time: {}", start.elapsed().as_secs_f64());
}
