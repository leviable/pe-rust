// By replacing the 1st digit of the 2-digit number *3, it turns out that six of the
// the nine possible values: 13, 23, 43, 53, 73, and 83, are all prime.
//
// By replacing the 3rd and 4th digits of 56**3 with the same digit, this 5-digits
// number is the first example having seven primes among the ten generated numbers,
// yielding the family: 56003, 56113, 56333, 56443, 56663, 56773, and 56993.
// Consequently 56003, being the first member of this family, is the smallest primes
// with this property.
//
// Find the smallest prime which, by replacing part of the number (not necessarily
// adjacent digits) with the same digit, is part of an eight prime value family.

use primes::{is_prime, PrimeSet, Sieve};
use std::time::Instant;

fn permutations(num_str: String) -> Vec<u64> {
    let mut nums: Vec<u64> = vec![];
    'outer: for x in 0..10 {
        let mut new_num: Vec<u64> = vec![];
        for (idx, val) in num_str.chars().enumerate() {
            if idx == 0 && x == 0 && val == '*' {
                continue 'outer;
            }
            match val {
                '*' => new_num.push(x),
                _ => new_num.push(val.to_digit(10).unwrap().into()),
            }
        }
        let y = new_num
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join("")
            .parse()
            .unwrap();
        if is_prime(y) {
            nums.push(y);
        }
    }

    nums.sort();
    nums
}

#[test]
fn test_permutations() {
    assert_eq!(
        permutations(String::from("*3")),
        vec![13, 23, 43, 53, 73, 83]
    );
    assert_eq!(
        permutations(String::from("56**3")),
        vec![56003, 56113, 56333, 56443, 56663, 56773, 56993]
    );
}

fn masks(num: u64) -> Vec<String> {
    let num: Vec<_> = num.to_string().chars().collect();
    let mask_bits = num.len() - 1;

    let mut masked_nums: Vec<String> = vec![];
    for mask in 1..(2_u32.pow(mask_bits as u32)) {
        let mut masked_num: Vec<char> = vec![];

        for (idx, mask_digit) in format!("{:0width$b}", mask, width = mask_bits)
            .chars()
            .enumerate()
        {
            match mask_digit {
                '0' => masked_num.push(num[idx]),
                '1' => masked_num.push('*'),
                _ => unreachable!(),
            }
        }
        masked_num.push(num[num.len() - 1]);
        masked_nums.push(String::from_iter(masked_num));
    }

    masked_nums
}

#[test]
fn test_masks() {
    assert_eq!(
        masks(123),
        vec!(
            String::from("1*3"),
            String::from("*23"),
            String::from("**3")
        )
    );
    assert_eq!(
        masks(1234),
        vec!(
            String::from("12*4"),
            String::from("1*34"),
            String::from("1**4"),
            String::from("*234"),
            String::from("*2*4"),
            String::from("**34"),
            String::from("***4"),
        )
    );
}

fn pe051() -> u64 {
    // Get a prime iterator
    // Count number of digits,
    // What is a good way to do permutations of the mask?
    0
}

fn main() {
    let start = Instant::now();
    println!("Time Elapsed: {}", start.elapsed().as_secs_f64());
}
