// A permutation is an ordered arrangement of objects. For example, 3124 is one
// possible permutation of the digits 1, 2, 3 and 4. If all of the permutations
// are listed numerically or alphabetically, we call it lexicographic order.
// The lexicographic permutations of 0, 1 and 2 are:

// 012   021   102   120   201   210

// What is the millionth lexicographic permutation of the digits 0, 1, 2, 3, 4, 5, 6, 7, 8 and 9?

use itertools::Itertools;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let digits = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut acc: Vec<Vec<_>> = vec![];
    for nums in digits.iter().permutations(digits.len()).unique() {
        acc.push(nums);
    }
    acc.sort();
    println!("1 Millionth Perm: {:?}", acc[999_999]);

    println!("Elapsed time: {}", start.elapsed().as_secs_f64());
}
