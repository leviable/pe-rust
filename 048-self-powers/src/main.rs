// The series,
// 1**1 + 2**2 + 3**3 + ... + 10**10 = 10405071317
//
// Find the last ten digits of the series,
// 1**1 + 2**2 + 3**3 + ... + 1000**1000 = X

use std::time::Instant;

fn trunc_add(num: u64, times: u64) -> u64 {
    let mut sum = 1;
    for _ in 0..times {
        sum *= num;
        sum %= 100_000_000_000;
    }
    sum
}

fn get_sum(limit: u64) -> u64 {
    let mut sum = 0;

    for num in 1..=limit {
        sum %= 100_000_000_000;
        sum += trunc_add(num, num);
    }

    sum
}

#[test]
fn test_get_sum() {
    assert_eq!(get_sum(10), 10405071317);
}

fn main() {
    let start = Instant::now();
    let solution = get_sum(1000);
    println!("Solution: {solution}");
    println!("Time elapsed: {}", start.elapsed().as_secs_f64());
}
