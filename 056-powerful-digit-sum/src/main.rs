// A googol (10**100) is a massive number: one followed by one-hundred
// zeros; 100**100 is almost unimaginably large: one followed by
// two-hundred zeros. Despite their size, the sum of the digits in each
// number is only 1.
//
// Considering natural numbers of the form,  a**b , where
// a,b < 100, what is the maximum digital sum?

use num_bigint::ToBigUint;
use std::time::Instant;

fn digit_sum(num: u32, pow: u32) -> u32 {
    let prod = num.to_biguint().unwrap().pow(pow);

    prod.to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .sum()
}

#[test]
fn test_big_pow() {
    assert_eq!(digit_sum(2, 2), 4);
    assert_eq!(digit_sum(3, 5), 9);
    assert_eq!(digit_sum(2, 99), 107);
}

fn pe056() -> u32 {
    let mut biggest = 0;
    for a in 1..100 {
        for b in 1..100 {
            let ans = digit_sum(a, b);
            if ans > biggest {
                biggest = ans;
            }
        }
    }

    biggest
}

fn main() {
    let start = Instant::now();
    let solution = pe056();
    println!("Solution: {solution}");
    println!("Time elapsed: {}", start.elapsed().as_secs_f64());
}
