// The following iterative sequence is defined for the set of positive integers:

// n → n/2 (n is even)
// n → 3n + 1 (n is odd)

// Using the rule above and starting with 13, we generate the following sequence:

// 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
// It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms. Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at 1.

// Which starting number, under one million, produces the longest chain?

// NOTE: Once the chain starts the terms are allowed to go above one million.

fn next_seq(num: usize) -> usize {
    let next = match num % 2 {
        0 => num / 2,
        1 => num * 3 + 1,
        _ => unreachable!(),
    };

    match next {
        1 => 2,
        _ => 1 + next_seq(next),
    }
}

#[test]
fn test_next_seq() {
    assert_eq!(next_seq(13), 10);
}

fn main() {
    let mut longest_chain = (0, 0);
    for num in 1..1_000_000 {
        let collatz_seq = next_seq(num);
        if collatz_seq > longest_chain.1 {
            longest_chain = (num, collatz_seq);
            println!("New longest {:?}", longest_chain);
        }
    }
}
