// Let d(n) be defined as the sum of proper divisors of n (numbers less than n
// which divide evenly into n). If d(a) = b and d(b) = a, where a ≠ b, then a
// and b are an amicable pair and each of a and b are called amicable numbers.

// For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22, 44, 55 and 110;
// therefore d(220) = 284. The proper divisors of 284 are 1, 2, 4, 71 and 142; so d(284) = 220.

// Evaluate the sum of all the amicable numbers under 10000.

use std::collections::{HashMap, HashSet};
use std::time::Instant;

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

fn main() {
    let start = Instant::now();
    let mut sum_map: HashMap<usize, usize> = HashMap::new();
    let mut sum_set: HashSet<usize> = HashSet::new();
    // Crunch divisors & sums for all numbers < 10_000
    for num in 1..10_000 {
        let divisor_sum = get_divisors(num).iter().sum();
        sum_map.insert(num, divisor_sum);
    }

    for (k, v) in sum_map.iter() {
        if v >= &10_000 || k == v {
            continue;
        }

        let value = match sum_map.get(v) {
            Some(value) => value,
            None => &0,
        };

        if value == k {
            sum_set.insert(*k);
        }
    }
    let sum: usize = sum_set.iter().sum();

    println!("Sum of all amicable pairs: {}", sum);
    println!("Elapsed time: {}", start.elapsed().as_secs_f64());
}
