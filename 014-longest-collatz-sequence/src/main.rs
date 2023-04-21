// The following iterative sequence is defined for the set of positive integers:

// n → n/2 (n is even)
// n → 3n + 1 (n is odd)

// Using the rule above and starting with 13, we generate the following sequence:

// 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
// It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms. Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at 1.

// Which starting number, under one million, produces the longest chain?

// NOTE: Once the chain starts the terms are allowed to go above one million.

use std::collections::HashMap;
// use std::ops::Add;

#[derive(Debug)]
struct Sequence {
    num: usize,
    chain_len: usize,
    hash_map: HashMap<usize, usize>,
}

// impl Add for Point {
//     type Output = Self;

//     fn add(self, other: Self) -> Self {
//         Self {
//             x: self.x + other.x,
//             y: self.y + other.y,
//         }
//     }
// }

fn next_seq(mut hash_map: HashMap<usize, usize>, num: usize) -> Sequence {
    if hash_map.contains_key(&num) {
        let chain_len = *hash_map.get(&num).unwrap();
        return Sequence {
            num,
            chain_len,
            hash_map,
        };
    }

    let next = match num % 2 {
        0 => num / 2,
        1 => num * 3 + 1,
        _ => unreachable!(),
    };

    &mut hash_map.insert(num, next);

    match next {
        1 => Sequence {
            num: num,
            chain_len: 2,
            hash_map,
        },
        _ => {
            let next_seq = next_seq(hash_map, next);
            Sequence {
                num: num,
                chain_len: next_seq.chain_len + 1,
                hash_map: next_seq.hash_map,
            }
        }
    }
}

#[test]
fn test_next_seq() {
    let mut hash_map: HashMap<usize, usize> = HashMap::new();
    let next = next_seq(hash_map, 13);
    assert_eq!(next.chain_len, 10);
}

fn longest_collatz_chain_under(limit: usize) -> Sequence {
    let mut hash_map: HashMap<usize, usize> = HashMap::new();

    let mut longest_seq: Sequence;

    for num in limit..=1 {
        let collatz_seq = next_seq(hash_map, num);
        hash_map = collatz_seq.hash_map;

        if &collatz_seq.chain_len > &longest_seq.chain_len {
            println!("New longest {:?}", longest_seq);
            longest_seq = collatz_seq;
        }
    }

    longest_seq
}

fn main() {
    let mut hash_map: HashMap<usize, usize> = HashMap::new();

    let mut longest_chain = (0, 0);

    for num in 1..1_000_000 {
        let collatz_seq = next_seq(hash_map, num);
        hash_map = collatz_seq.hash_map;

        if collatz_seq.chain_len > longest_chain.1 {
            longest_chain = (collatz_seq.num, collatz_seq.chain_len);
            println!("New longest {:?}", longest_chain);
        }
    }
}
