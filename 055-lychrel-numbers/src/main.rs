// If we take 47, reverse and add, 47 + 74 = 121, which is palindromic.
//
// Not all numbers produce palindromes so quickly. For example,
//
// 349 + 943 = 1292
// 1292 + 2921 = 4213
// 4213 + 3124 = 7337
//
// That is, 349 took three iterations to arrive at a palindrome.
//
// Although no one has proved it yet, it is thought that some numbers, like 196,
// never produce a palindrome. A number that never forms a palindrome through the
// reverse and add process is called a Lychrel number. Due to the theoretical nature
// of these numbers, and for the purpose of this problem, we shall assume that a
// number is Lychrel until proven otherwise. In addition you are given that for
// every number below ten-thousand, it will either (i) become a palindrome in less
// than fifty iterations, or, (ii) no one, with all the computing power that exists,
// has managed so far to map it to a palindrome. In fact, 10677 is the first number
// to be shown to require over fifty iterations before producing a palindrome:
// 4668731596684224866951378664 (53 iterations, 28-digits).
//
// Surprisingly, there are palindromic numbers that are themselves Lychrel numbers;
// the first example is 4994.
//
// How many Lychrel numbers are there below ten-thousand?
//
// NOTE: Wording was modified slightly on 24 April 2007 to emphasise the theoretical nature of Lychrel numbers.

use std::time::Instant;

fn palindromic(num: u128) -> bool {
    num == reverse(num)
}

#[test]
fn test_palindromic() {
    assert_eq!(palindromic(121), true);
    assert_eq!(palindromic(349), false);
    assert_eq!(palindromic(4994), true);
}

fn reverse(num: u128) -> u128 {
    num.to_string()
        .chars()
        .rev()
        .collect::<String>()
        .parse()
        .unwrap()
}

#[test]
fn test_reverse() {
    assert_eq!(reverse(47), 74);
    assert_eq!(reverse(349), 943);
    assert_eq!(reverse(1292), 2921);
    assert_eq!(reverse(4994), 4994);
}

fn lychrel_number(num: u128, idx: u128) -> bool {
    let num = num + reverse(num);
    if idx >= 50 {
        true
    } else if !palindromic(num) {
        lychrel_number(num, idx + 1)
    } else {
        false
    }
}

#[test]
fn test_lychrel_number() {
    assert_eq!(lychrel_number(47, 0), false);
    assert_eq!(lychrel_number(349, 0), false);
    assert_eq!(lychrel_number(196, 0), true);
    assert_eq!(lychrel_number(4_994, 0), true);
    // assert_eq!(lychrel_number(10_677, 0), false);
}

fn pe055() -> i32 {
    (1u128..10_000).filter(|x| lychrel_number(*x, 0)).count() as i32
}

fn main() {
    let start = Instant::now();
    let solution = pe055();
    println!("PE-055: {}", solution);
    println!("Time elapsed: {}", start.elapsed().as_secs_f64());
}
