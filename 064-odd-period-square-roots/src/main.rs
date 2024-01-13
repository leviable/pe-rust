// All square roots are periodic when written as continued fractions and can be written in the form:
//
// For example, let us consider
//
// If we continue we would get the following expansion:

// The process can be summarised as follows:
//
// It can be seen that the sequence is repeating. For conciseness, we use the notation
// , to indicate that the block (1,3,1,8) repeats indefinitely.
//
// The first ten continued fraction representations of (irrational) square roots are:
//
// Exactly four continued fractions, for
// , have an odd period.
//
// How many continued fractions for
//  have an odd period?
//

use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
enum Period {
    Odd,
    Even,
}

#[derive(Debug, PartialEq, Eq)]
struct SquareRoot {
    num: u64,
    expansion: Vec<u64>,
    period: Period,
}

fn get_square_root(num: u64) -> SquareRoot {
    let mut hs: HashSet<String> = HashSet::new();
    let mut expansion: Vec<u64> = vec![];

    let sqrt = (num as f64).sqrt();
    let mut integer_part = sqrt as u64;
    expansion.push(integer_part);
    let mut fraction_part = sqrt - integer_part as f64;
    hs.insert(((fraction_part * 10000.0) as u64).to_string());

    loop {
        let base = fraction_part.recip();
        integer_part = base as u64;
        expansion.push(integer_part);
        fraction_part = base - integer_part as f64;

        if hs.contains(&((fraction_part * 10000.0) as u64).to_string()) {
            break;
        } else {
            hs.insert(((fraction_part * 10000.0) as u64).to_string());
        }
    }

    let period = if (expansion.len() - 1) % 2 == 1 {
        Period::Odd
    } else {
        Period::Even
    };

    SquareRoot {
        num,
        period,
        expansion,
    }
}

#[test]
fn test_get_square_root() {
    assert_eq!(
        get_square_root(2),
        SquareRoot {
            num: 2,
            period: Period::Odd,
            expansion: vec![1, 2]
        }
    );
    assert_eq!(
        get_square_root(3),
        SquareRoot {
            num: 3,
            period: Period::Even,
            expansion: vec![1, 1, 2]
        }
    );
    assert_eq!(
        get_square_root(5),
        SquareRoot {
            num: 5,
            period: Period::Odd,
            expansion: vec![2, 4]
        }
    );
    assert_eq!(
        get_square_root(6),
        SquareRoot {
            num: 6,
            period: Period::Even,
            expansion: vec![2, 2, 4]
        }
    );
    assert_eq!(
        get_square_root(7),
        SquareRoot {
            num: 7,
            period: Period::Even,
            expansion: vec![2, 1, 1, 1, 4],
        }
    );
    assert_eq!(
        get_square_root(8),
        SquareRoot {
            num: 8,
            period: Period::Even,
            expansion: vec![2, 1, 4],
        }
    );
    assert_eq!(
        get_square_root(10),
        SquareRoot {
            num: 10,
            period: Period::Odd,
            expansion: vec![3, 6],
        }
    );
    assert_eq!(
        get_square_root(11),
        SquareRoot {
            num: 11,
            period: Period::Even,
            expansion: vec![3, 3, 6],
        }
    );
    assert_eq!(
        get_square_root(12),
        SquareRoot {
            num: 12,
            period: Period::Even,
            expansion: vec![3, 2, 6],
        }
    );
    assert_eq!(
        get_square_root(13),
        SquareRoot {
            num: 13,
            period: Period::Odd,
            expansion: vec![3, 1, 1, 1, 1, 6],
        }
    );
    assert_eq!(
        get_square_root(23),
        SquareRoot {
            num: 23,
            period: Period::Even,
            expansion: vec![4, 1, 3, 1, 8],
        }
    );
}

fn pe064(limit: u64) -> u64 {
    let squares = (1..=limit).map(|x| x.pow(2)).collect::<Vec<_>>();

    let mut count = 0u64;
    for num in 2..=limit {
        if squares.contains(&num) {
            continue;
        }
        let sqrt = get_square_root(num);

        // eprintln!("111111111111111111111111111111111111111111111111111");
        // eprintln!("{:?}", sqrt);

        if sqrt.period == Period::Odd {
            count += 1
        }
    }

    count
}

#[test]
fn test_pe064() {
    assert_eq!(pe064(13), 4);
}

fn main() {
    let start = std::time::Instant::now();

    let solution = pe064(10_000);
    println!("Solution: {solution}");
    println!("Elapsed time: {}", start.elapsed().as_secs_f64());
}
