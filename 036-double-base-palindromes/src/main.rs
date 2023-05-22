// The decimal number, 585 = 10010010012 (binary), is palindromic in both bases.

// Find the sum of all numbers, less than one million, which are palindromic in base 10 and base 2.

// (Please note that the palindromic number, in either base, may not include leading zeros.)

use std::time::Instant;

fn is_pal_base_10(num: usize) -> bool {
    num == num
        .to_string()
        .chars()
        .rev()
        .collect::<String>()
        .parse()
        .unwrap()
}

#[test]
fn test_is_pal_base_10() {
    assert!(is_pal_base_10(585));
    assert!(is_pal_base_10(55));
    assert!(!is_pal_base_10(586));
}

fn is_pal_base_2(base10: usize) -> bool {
    let base2 = format!("{base10:b}");

    base2 == base2.chars().rev().collect::<String>()
}

#[test]
fn test_is_pal_base_2() {
    assert!(is_pal_base_2(585));
    assert!(!is_pal_base_2(586));
}

fn is_double_palindrome(num: usize) -> bool {
    is_pal_base_10(num) && is_pal_base_2(num)
}

#[test]
fn test_is_double_palindrome() {
    assert!(is_double_palindrome(585));
}

fn pe_36() -> usize {
    let mut pals: Vec<usize> = vec![];
    for num in 1..1_000_000 {
        if is_double_palindrome(num) {
            pals.push(num);
        }
    }

    pals.iter().sum()
}

fn main() {
    let start = Instant::now();
    let solution = pe_36();
    println!("Solution: {}", solution);
    println!("Elapsed time: {}", start.elapsed().as_secs_f64());
}
