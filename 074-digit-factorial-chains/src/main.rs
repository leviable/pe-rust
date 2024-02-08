// The number is well known for the property that the sum of the factorial of its digits is equal to :
//
// Perhaps less well known is , in that it produces the longest chain of numbers that link back to ;
//     it turns out that there are only three such loops that exist:
//
//
// It is not difficult to prove that EVERY starting number will eventually get stuck
//     in a loop. For example,
//
//
// Starting with produces a chain of five non-repeating terms, but the longest non-repeating chain
//         with a starting number below one million is sixty terms.
//
// How many chains, with a starting number below one million, contain exactly sixty non-repeating terms?
use std::collections::HashSet;

fn chain_length(num: u64) -> u64 {
    let mut hs: HashSet<u64> = HashSet::new();

    let mut digit_fac = digit_factorial(num);

    while !hs.contains(&digit_fac) {
        hs.insert(digit_fac);

        digit_fac = digit_factorial(digit_fac);
    }

    hs.insert(num);

    hs.len() as u64
}

#[test]
fn test_chain_length() {
    assert_eq!(chain_length(69), 5);
    assert_eq!(chain_length(78), 4);
    assert_eq!(chain_length(540), 2);
}

fn digit_factorial(num: u64) -> u64 {
    num.to_string()
        .chars()
        .map(|x| factorial(x.to_digit(10).unwrap() as u64))
        .sum()
}

#[test]
fn test_digit_factorial() {
    assert_eq!(digit_factorial(145), 145);
    assert_eq!(digit_factorial(169), 363601);
    assert_eq!(digit_factorial(871), 45361);
}

fn factorial(num: u64) -> u64 {
    let mut product = 1;

    for n in 1..=num {
        product *= n
    }

    product
}

#[test]
fn test_factorial() {
    assert_eq!(factorial(1), 1);
    assert_eq!(factorial(2), 2);
    assert_eq!(factorial(3), 6);
    assert_eq!(factorial(4), 24);
}
fn pe074() -> u64 {
    let mut count = 0;

    for num in 1..1_000_000 {
        let chain_len = chain_length(num);
        if chain_len == 60 {
            count += 1;
        }
    }

    count
}

fn main() {
    let start = std::time::Instant::now();
    let solution = pe074();
    println!("Solution: {solution}");
    println!("Elapsed time: {}", start.elapsed().as_secs_f64());
}
