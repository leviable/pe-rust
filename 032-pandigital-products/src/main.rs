// We shall say that an n-digit number is pandigital if it makes use of all the
// digits 1 to n exactly once; for example, the 5-digit number, 15234, is 1 through
// 5 pandigital.

// The product 7254 is unusual, as the identity, 39 × 186 = 7254, containing
// multiplicand, multiplier, and product is 1 through 9 pandigital.

// Find the sum of all products whose multiplicand/multiplier/product identity can
// be written as a 1 through 9 pandigital.

// HINT: Some products can be obtained in more than one way so be sure to only
// include it once in your sum.

use itertools::Itertools;
use std::collections::HashSet;
use std::time::Instant;

fn is_pandigital(num: usize) -> bool {
    let mut num_chars = num.to_string().chars().collect::<Vec<char>>();
    num_chars.sort();

    num_chars == ['1', '2', '3', '4', '5', '6', '7', '8', '9'][..num_chars.len()]
}

#[test]
fn test_is_pandigital() {
    assert_eq!(is_pandigital(12354), true);
    assert_eq!(is_pandigital(12344), false);
    assert_eq!(is_pandigital(12346), false);
}

fn are_pandigital(num1: usize, num2: usize, num3: usize) -> bool {
    let as_str = format!("{}{}{}", num1, num2, num3);
    let as_num = as_str.parse().unwrap();

    is_pandigital(as_num)
}

#[test]
fn test_are_pandigital() {
    assert_eq!(are_pandigital(1, 2, 3), true);
    assert_eq!(are_pandigital(1, 2, 4), false);
    assert_eq!(are_pandigital(13, 24, 65), true);
}

fn check_perm(perm: Vec<&usize>) -> Vec<usize> {
    let mut solutions: Vec<usize> = vec![];

    for left_idx in 1..5 {
        for right_idx in 1..5 {
            let left: &usize = &perm[0..left_idx]
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join("")
                .parse()
                .unwrap();

            let right: &usize = &perm[left_idx..(left_idx + right_idx)]
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join("")
                .parse()
                .unwrap();
            let prod: &usize = &perm[(left_idx + right_idx)..]
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join("")
                .parse()
                .unwrap();

            if left * right == *prod && are_pandigital(*left, *right, *prod) {
                solutions.push(*prod);
            }
        }
    }

    solutions
}

fn p32() -> usize {
    let possible_nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut set: HashSet<usize> = HashSet::new();

    for perm in possible_nums.iter().permutations(9) {
        for solution in check_perm(perm) {
            set.insert(solution);
        }
    }

    set.iter().sum()
}

fn main() {
    let start = Instant::now();
    let solution = p32();
    println!("Sum of products: {solution}");
    println!("Elapsed Time: {}", start.elapsed().as_secs_f64());
}
