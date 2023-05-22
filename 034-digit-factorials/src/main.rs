// 145 is a curious number, as 1! + 4! + 5! = 1 + 24 + 120 = 145.

// Find the sum of all numbers which are equal to the sum of the factorial of their digits.

// Note: As 1! = 1 and 2! = 2 are not sums they are not included.

use std::time::Instant;

fn factorial(num: u32) -> u32 {
    (1..=num).product()
}

fn is_digit_factorial(num: u32) -> bool {
    // Split number into digits
    // take factorial of each digit then sum
    let num_str = &num.to_string();
    let num_digits: Vec<u32> = num_str.chars().map(|x| x.to_digit(10).unwrap()).collect();
    // let num_sum = num_digits.iter().map(|x| factorial(*x)).sum();
    let num_sum = num_digits.iter().map(|x| factorial(*x)).sum::<u32>();

    num == num_sum
}

#[test]
fn test_is_digit_factorial() {
    assert!(is_digit_factorial(145), "Expected 145 to be true");
    assert!(!is_digit_factorial(144), "Expected 144 to be false");
}

fn pe034() -> usize {
    let mut solution = 0;
    for num in 3..99999 {
        if is_digit_factorial(num) {
            solution += num
        }
    }

    solution as usize
}

fn main() {
    let start = Instant::now();
    let solution = pe034();
    println!("Solution is: {solution}");
    println!("Time elapsed: {}", start.elapsed().as_secs_f64());
}
