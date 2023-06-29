// The arithmetic sequence, 1487, 4817, 8147, in which each of the terms increases by 3330,
// is unusual in two ways:
//  (i)  each of the three terms are prime, and,
//  (ii) each of the 4-digit numbers are permutations of one another.
//
// There are no arithmetic sequences made up of three 1-, 2-, or 3-digit primes,
// exhibiting this property, but there is one other 4-digit increasing sequence.
//
// What 12-digit number do you form by concatenating the three terms in this sequence?

use itertools::Itertools;
use std::time::Instant;

fn get_perms(prime: u64) -> Vec<u64> {
    // Cast to string,
    // split chars
    // collect?
    // do perms on char vec
    // collect
    // for each, recombine into uint
    prime
        .to_string()
        .split("")
        .permutations(3)
        .unique()
        .map(|x| x.join("").parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}

#[test]
fn test_foo() {
    println!("111111111111111111111111111111111111111");
    println!(
        "{:?}",
        123.to_string()
            .chars()
            .permutations(3)
            .collect::<Vec<_>>()
            .iter() // .map(|x| x.parse())
            .map(|x| x.into_iter().collect::<String>())
            .collect::<Vec<_>>()
            .iter()
            .map(|y| y.parse::<u64>().unwrap())
            .collect::<Vec<u64>>()
    );
    println!("111111111111111111111111111111111111111");
    assert_eq!(1, 2);
}

#[test]
fn test_bar() {
    let items = vec![0, 0, 1, 2];
    for perm in items.iter().permutations(items.len()).unique() {
        println!("{:?}", perm);
    }
    assert_eq!(1, 2);
}

#[test]
fn test_get_perms() {
    assert_eq!(get_perms(123), vec![123, 132, 213, 231, 312, 321]);
}

fn pe049() -> u64 {
    0
}

fn main() {
    let start = Instant::now();
    let solution = pe049();
    println!("Solution: {:?}", solution);
    println!("Time elapsed: {}", start.elapsed().as_secs_f64());
}
