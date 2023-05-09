// Surprisingly there are only three numbers that can be written as the sum of fourth powers of their digits:

// 1634 = 14 + 64 + 34 + 44
// 8208 = 84 + 24 + 04 + 84
// 9474 = 94 + 44 + 74 + 44
// As 1 = 14 is not a sum it is not included.

// The sum of these numbers is 1634 + 8208 + 9474 = 19316.

// Find the sum of all the numbers that can be written as the sum of fifth powers of their digits.

use std::time::Instant;

fn sum_of_powers(pow: u32, num: u32) -> u32 {
    num.to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap().pow(pow))
        .sum()
}

#[test]
fn test_sum_of_powers() {
    assert_eq!(sum_of_powers(4, 1634), 1634);
    assert_eq!(sum_of_powers(4, 8208), 8208);
    assert_eq!(sum_of_powers(4, 9474), 9474);
}

fn is_sum_of_powers(pow: u32, num: u32) -> bool {
    num == sum_of_powers(pow, num)
}

fn find_all_sops(pow: u32) -> Vec<u32> {
    let mut sops = vec![];
    for x in 10_u32.pow(pow - 2)..10_u32.pow(pow + 1) {
        if is_sum_of_powers(pow, x) {
            println!("Found one: {}", x);
            sops.push(x);
        }
    }

    sops
}

#[test]
fn test_find_all_sops() {
    assert_eq!(find_all_sops(4), vec![1634, 8208, 9474]);
}

#[test]
fn test_is_sum_of_powers() {
    assert_eq!(is_sum_of_powers(4, 1634), true);
    assert_eq!(is_sum_of_powers(4, 8208), true);
    assert_eq!(is_sum_of_powers(4, 9474), true);
    assert_eq!(is_sum_of_powers(4, 9999), false);
}

fn main() {
    let start = Instant::now();
    let sops = find_all_sops(5);
    println!("sop is: {}", sops.iter().sum::<u32>());
    println!("Elapsed time: {}", start.elapsed().as_secs_f64());
}
