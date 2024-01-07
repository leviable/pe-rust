// A number chain is created by continuously adding the square of the digits in a
// number to form a new number until it has been seen before.

// For example,
//
// Therefore any chain that arrives at
//  or
//  will become stuck in an endless loop. What is most amazing is that EVERY starting
//  number will eventually arrive at
//  or
// .
//
// How many starting numbers below ten million will arrive at
// ?

#[derive(PartialEq, Eq, Debug)]
enum Chain {
    One,
    EightyNine,
}

fn run_chain(start: u64) -> Chain {
    match num_sum(start) {
        1 => Chain::One,
        89 => Chain::EightyNine,
        x => run_chain(x),
    }
}

fn num_sum(num: u64) -> u64 {
    num.to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as u64)
        .map(|y| y.pow(2))
        .sum::<u64>()
}

fn pe092(limit: u64) -> u64 {
    (1u64..limit)
        .map(run_chain)
        .filter(|x| *x == Chain::EightyNine)
        .count()
        .try_into()
        .unwrap()
}

fn main() {
    let start = std::time::Instant::now();

    let solution = pe092(10_000_000);
    println!("Solution: {solution}");
    println!("Time elapsed: {}", start.elapsed().as_secs_f64());
}

#[test]
fn test_chain() {
    assert_eq!(run_chain(44), Chain::One);
    assert_eq!(run_chain(8), Chain::EightyNine);
}

#[test]
fn test_num_sum() {
    assert_eq!(num_sum(44), 32);
    assert_eq!(num_sum(32), 13);
}
