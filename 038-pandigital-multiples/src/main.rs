// Take the number 192 and multiply it by each of 1, 2, and 3:

// 192 × 1 = 192
// 192 × 2 = 384
// 192 × 3 = 576
// By concatenating each product we get the 1 to 9 pandigital, 192384576. We will
// call 192384576 the concatenated product of 192 and (1,2,3)

// The same can be achieved by starting with 9 and multiplying by 1, 2, 3, 4, and 5,
// giving the pandigital, 918273645, which is the concatenated product of 9 and (1,2,3,4,5).

// What is the largest 1 to 9 pandigital 9-digit number that can be formed as the
// concatenated product of an integer with (1,2, ... , n) where n > 1?

use std::time::Instant;

const PANDIGITAL: &str = "123456789";

fn is_pandigital(mut num: Vec<u32>) -> bool {
    num.sort();
    PANDIGITAL
        == num
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join("")
}

fn get_pandigital_product(num: u64) -> Option<u64> {
    let mut product: Vec<u32> = vec![];
    let mut count = 0;
    while product.len() < 9 {
        count += 1;
        let prod = num * count;
        _ = prod
            .to_string()
            .chars()
            .map(|x| product.push(x.to_digit(10).unwrap()))
            .collect::<Vec<_>>();
    }
    if !is_pandigital(product.clone()) {
        return None;
    }
    match &product.len() {
        9 => Some(
            product
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join("")
                .parse()
                .unwrap(),
        ),
        _ => None,
    }
}

#[test]
fn test_get_pandigital_product() {
    match get_pandigital_product(192) {
        Some(x) => assert_eq!(x, 192384576),
        None => assert!(false),
    };
    match get_pandigital_product(194) {
        Some(_x) => assert!(false),
        None => assert!(true),
    };
}

#[test]
fn test_is_pandigital() {
    assert!(is_pandigital(vec![1, 9, 2, 3, 8, 4, 5, 7, 6]));
    assert!(!is_pandigital(vec![9, 9, 2, 3, 8, 4, 5, 7, 6]));
}

fn pe038() -> u64 {
    let mut largest = 0;
    for num in 1..10_000 {
        match get_pandigital_product(num) {
            Some(x) => {
                if x > largest {
                    largest = x;
                    println!("New largest: {largest} at {num}");
                }
            }
            None => (),
        };
    }
    largest
}

fn main() {
    let start = Instant::now();
    let solution = pe038();
    println!("Solution: {}", solution);
    println!("Time elapsed: {}", start.elapsed().as_secs_f64());
}
