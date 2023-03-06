// The sum of the squares of the first ten natural numbers is,

// The square of the sum of the first ten natural numbers is,

// Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is
// .

// Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

fn main() {
    println!("Difference is: {}", get_difference(100));
}

fn get_difference(upper_bound: u32) -> u32 {
    get_square_of_sum(upper_bound) - get_sum_of_squares(upper_bound)
}

#[test]
fn test_get_difference() {
    assert_eq!(get_difference(1), 0);
    assert_eq!(get_difference(2), 4);
    assert_eq!(get_difference(10), 2640);
}

fn get_square_of_sum(upper_bound: u32) -> u32 {
    (1..(upper_bound + 1)).sum::<u32>().pow(2)
}

#[test]
fn test_get_square_of_sum() {
    assert_eq!(get_square_of_sum(2), 9);
    assert_eq!(get_square_of_sum(3), 36);
    assert_eq!(get_square_of_sum(4), 100);
}

fn get_sum_of_squares(upper_bound: u32) -> u32 {
    let mut total: u32 = 0;
    for num in 1..upper_bound + 1 {
        total += num.pow(2);
    }

    total
}

#[test]
fn test_get_sum_of_squares() {
    assert_eq!(get_sum_of_squares(2), 5);
    assert_eq!(get_sum_of_squares(3), 14);
    assert_eq!(get_sum_of_squares(4), 30);
}
