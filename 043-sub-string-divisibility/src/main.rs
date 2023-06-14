// The number, 1406357289, is a 0 to 9 pandigital number because it is made up of each of the
// digits 0 to 9 in some order, but it also has a rather interesting sub-string divisibilityproperty
// property.
//
// Let d1 be the 1st digit, d2 be the 2nd digit, and so on. In this way, we note the following:
//
//  d2d3d4 is divisible by 2
//  d3d4d5 is divisible by 3
//  d4d5d6 is divisible by 5
//  d5d6d7 is divisible by 7
//  d6d7d8 is divisible by 11
//  d7d8d9 is divisible by 13
//  d8d9d10 is divisible by 17
//
// Find the sum of all 0 to 9 pandigital numbers with this property.

use itertools::Itertools;
use std::time::Instant;

fn get_permutations(nums: &Vec<usize>) -> Vec<Vec<&usize>> {
    nums.iter().permutations(nums.len()).collect_vec()
}

#[test]
fn test_get_permutations() {
    let nums: Vec<usize> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let perms = get_permutations(&nums);
    assert_eq!(perms.len(), 3628800);
}

fn is_substring_divisible(sub: &Vec<&usize>) -> bool {
    let primes = vec![2, 3, 5, 7, 11, 13, 17];

    for (idx, count) in (1..8).enumerate() {
        let sub_num = format!("{}{}{}", sub[count], sub[count + 1], sub[count + 2])
            .parse::<usize>()
            .unwrap();
        if sub_num % primes[idx] != 0 {
            return false;
        }
    }
    true
}

#[test]
fn test_is_substring_divisible() {
    let mut v: Vec<&usize> = vec![&0, &1, &2, &3, &4, &5, &6, &7, &8, &9];
    assert_eq!(is_substring_divisible(&v), false);
    v = vec![&1, &4, &0, &6, &3, &5, &7, &2, &8, &9];
    assert_eq!(is_substring_divisible(&v), true);
}

fn pe043() -> usize {
    let nums: Vec<usize> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut total: usize = 0;

    for perm in get_permutations(&nums) {
        if is_substring_divisible(&perm) {
            total += perm
                .iter()
                .map(|x| format!("{x}"))
                .collect::<String>()
                .parse::<usize>()
                .unwrap();
        }
    }
    total
}

fn main() {
    let start = Instant::now();
    let sum = pe043();
    println!("The solution is: {sum}");
    println!("Elapsed time: {}", start.elapsed().as_secs_f64());
}
