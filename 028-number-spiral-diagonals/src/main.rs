// Starting with the number 1 and moving to the right in a clockwise direction
// a 5 by 5 spiral is formed as follows:

// 21 22 23 24 25
// 20  7  8  9 10
// 19  6  1  2 11
// 18  5  4  3 12
// 17 16 15 14 13

// It can be verified that the sum of the numbers on the diagonals is 101.

// What is the sum of the numbers on the diagonals in a 1001 by 1001 spiral formed
// in the same way?

use std::time::Instant;

fn get_diag_sum(grid_size: usize) -> usize {
    let mut sum = 0;
    let mut scalar = 2;
    let mut count = 0;

    for idx in 1..=(grid_size.pow(2)) {
        if idx == 1 {
            sum += 1;
            continue;
        } else if (idx - 1) % scalar == 0 {
            sum += idx;
            count = (count + 1) % 4;
            if count == 0 {
                scalar += 2;
            }
        }
    }

    sum
}

#[test]
fn test_get_diag_sum() {
    assert_eq!(get_diag_sum(1), 1);
    assert_eq!(get_diag_sum(3), 25);
    assert_eq!(get_diag_sum(5), 101);
    assert_eq!(get_diag_sum(7), 261);
}

fn main() {
    let start = Instant::now();
    let sum = get_diag_sum(1001);
    println!("Sum of diagnoals for 1001 grid is: {}", sum);
    println!("Elapsed time: {}", start.elapsed().as_secs_f64());
}
