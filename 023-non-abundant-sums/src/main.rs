// A perfect number is a number for which the sum of its proper divisors is exactly
// equal to the number. For example, the sum of the proper divisors of 28 would be
// 1 + 2 + 4 + 7 + 14 = 28, which means that 28 is a perfect number.

// A number n is called deficient if the sum of its proper divisors is less than
// and it is called abundant if this sum exceeds n.

// As 12 is the smallest abundant number, 1 + 2 + 3 + 4 + 6 = 16, the smallest number
// that can be written as the sum of two abundant numbers is 24. By mathematical analysis,
// it can be shown that all integers greater than 28123 can be written as the sum of
// two abundant numbers. However, this upper limit cannot be reduced any further by
// analysis even though it is known that the greatest number that cannot be expressed
// as the sum of two abundant numbers is less than this limit.

// Find the sum of all the positive integers which cannot be written as the sum of
// two abundant numbers.

use std::cmp::Ordering;
use std::collections::HashSet;
use std::time::Instant;

#[derive(Debug, PartialEq)]
enum NumberType {
    Deficient,
    Perfect,
    Abundant,
}

fn get_number_type(num: usize) -> NumberType {
    let div = get_divisors(num);

    match div.iter().sum::<usize>().cmp(&num) {
        Ordering::Less => NumberType::Deficient,
        Ordering::Greater => NumberType::Abundant,
        Ordering::Equal => NumberType::Perfect,
    }
}

#[test]
fn test_get_number_type() {
    assert_eq!(get_number_type(28), NumberType::Perfect);
    assert_eq!(get_number_type(10), NumberType::Deficient);
    assert_eq!(get_number_type(12), NumberType::Abundant);
}

fn get_divisors(num: usize) -> Vec<usize> {
    let mut divisors: Vec<usize> = vec![];
    for div in 1..num {
        if num % div == 0 {
            divisors.push(div)
        }
    }
    divisors
}

#[test]
fn test_get_divisors() {
    assert_eq!(
        get_divisors(220),
        vec!(1, 2, 4, 5, 10, 11, 20, 22, 44, 55, 110)
    );
    assert_eq!(get_divisors(284), vec!(1, 2, 4, 71, 142));
}

fn get_abundant_nums(limit: usize) -> Vec<usize> {
    let mut nums: Vec<usize> = vec![];
    for num in 1..limit {
        if get_number_type(num) == NumberType::Abundant {
            nums.push(num);
        }
    }
    nums
}

#[test]
fn test_get_abundant_nums() {
    assert_eq!(get_abundant_nums(20), vec![12, 18]);
}

fn main() {
    let start = Instant::now();
    let mut num_pool: HashSet<usize> = HashSet::from_iter(1..28123);
    let abundant_nums = get_abundant_nums(28123);

    for x in 0..(abundant_nums.len()) {
        for y in x + 0..(abundant_nums.len()) {
            let s = abundant_nums[x] + abundant_nums[y];
            if s > 28133 {
                continue;
            }
            num_pool.remove(&s);
        }
    }

    let total: usize = num_pool.iter().sum();
    println!("Total: {total}");
    println!("Elapsed time: {}", start.elapsed().as_secs_f64());
}
