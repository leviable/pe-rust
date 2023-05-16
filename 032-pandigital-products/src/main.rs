// We shall say that an n-digit number is pandigital if it makes use of all the
// digits 1 to n exactly once; for example, the 5-digit number, 15234, is 1 through
// 5 pandigital.

// The product 7254 is unusual, as the identity, 39 × 186 = 7254, containing
// multiplicand, multiplier, and product is 1 through 9 pandigital.

// Find the sum of all products whose multiplicand/multiplier/product identity can
// be written as a 1 through 9 pandigital.

// HINT: Some products can be obtained in more than one way so be sure to only
// include it once in your sum.

use std::time::Instant;

fn is_pandigital(num: usize) -> bool {
    let mut num_chars = num.to_string().chars().collect::<Vec<char>>();
    num_chars.sort();

    num_chars == ['1', '2', '3', '4', '5', '6', '7', '8', '9'][..num_chars.len()]
}

#[test]
fn test_is_pandigital() {
    assert_eq!(is_pandigital(12354), true);
    assert_eq!(is_pandigital(12344), false);
    assert_eq!(is_pandigital(12346), false);
}

fn are_pandigital(num1: usize, num2: usize, num3: usize) -> bool {
    let as_str = format!("{}{}{}", num1, num2, num3);
    let as_num = as_str.parse().unwrap();

    is_pandigital(as_num)
}

#[test]
fn test_are_pandigital() {
    assert_eq!(are_pandigital(1, 2, 3), true);
    assert_eq!(are_pandigital(1, 2, 4), false);
    assert_eq!(are_pandigital(13, 24, 65), true);
}

fn is_too_large(num1: usize, num2: usize, num3: usize) -> bool {
    let num3_size = num1.to_string().len() + num2.to_string().len();

    num3 >= 9 * num3_size
}

#[test]
fn test_is_too_large() {
    assert_eq!(is_too_large(1, 2, 3), false);
    assert_eq!(is_too_large(98, 7, 686), true);
}

fn main() {
    let start = Instant::now();
    let mut total = 0;
    'outer: for a in 1.. {
        println!("111111111111111111111111111 {a}");
        for b in 1.. {
            println!("2222222222222222222222222222 {b}");
            let c = a * b;
            if is_too_large(a, b, c) {
                continue 'outer;
            } else if are_pandigital(a, b, c) {
                println!("Found one: {a} * {b} = {c}");
                total += c;
            }
        }
    }
    println!("Sum of products: {total}");
    println!("Elapsed Time: {}", start.elapsed().as_secs_f64());
}
