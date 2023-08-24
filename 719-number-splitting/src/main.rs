// We define an S-number to be a natural number, n, that is a perfect square and
// its square root can be obtained by splitting the decimal representation of n
// into 2 or more numbers then adding the numbers.
//
// For example, 81 is an S-number because  sqrt(81) = 8 + 1
//  6724 is an S-number:  sqrt(6724) =  6 + 72 + 4
//  8281 is an S-number:  sqrt(8281) = 82 +  8 + 1
//  9801 is an S-number:  sqrt(9801) = 98 +  0 + 1
//
// Further we define  T(N) to be the sum of all  S numbers n <= N. You are given
// T(10**4) == 41333
//
// Find  T(10**12)

// NOTE: can maybe do a recursive function here to split the number and get
//       permutations

use itertools::Itertools;

fn variants_of(num: u64) -> Vec<Vec<u64>> {
    let mut combs = vec![];
    let num = num.to_string().chars().collect::<Vec<_>>();

    if num.len() == 1 {
        return vec![vec![num[0].to_digit(10).unwrap() as u64]];
    }

    for comb_count in 2..=num.len() {
        for comb in (0..num.len()).combinations(comb_count) {
            if comb[0] != 0 {
                continue;
            }
            combs.push(recurse(num.clone(), comb));
            // println!("{:?}", combs);
        }
    }
    combs
}

fn recurse(num: Vec<char>, comb: Vec<usize>) -> Vec<u64> {
    // println!("{:?} -> {:?}", num, comb);
    match comb.len() {
        // Reached the end of our recursion. Combine the remaning nums into one num, and
        // return it as a vector of len 1
        1 => vec![num[comb[0]..].iter().join("").parse::<u64>().unwrap()],
        _ => {
            let from_recurse = recurse(num.clone(), comb.clone()[1..].to_vec());
            let mut next = vec![num[comb[0]..comb[1]]
                .iter()
                .join("")
                .parse::<u64>()
                .unwrap()];
            next.extend(from_recurse);
            next
        }
    }
}

#[test]
fn test_variants_of() {
    assert_eq!(variants_of(9), vec![vec![9]]);
    assert_eq!(variants_of(25), vec![vec![2, 5]]);
    assert_eq!(
        variants_of(100),
        vec![vec![1, 0], vec![10, 0], vec![1, 0, 0]]
    );
}
fn s_number(root: u64, squared: u64) -> bool {
    let vars = variants_of(squared);
    vars.iter().any(|x| x.iter().sum::<u64>() == root)
}

#[test]
fn test_s_number() {
    assert_eq!(s_number(9, 81), true);
    assert_eq!(s_number(82, 6724), true);
    assert_eq!(s_number(91, 8281), true);
    assert_eq!(s_number(99, 9801), true);
    assert_eq!(s_number(5, 25), false);
}

fn pe719(limit: u64) -> u64 {
    let mut s_nums: Vec<u64> = vec![];
    for root in 2u64.. {
        let squared = root.pow(2);
        println!(
            "Sum: {} -> working on: {squared}",
            s_nums.iter().sum::<u64>()
        );
        if squared > limit {
            break;
        }
        if s_number(root, squared) {
            s_nums.push(squared);
        }
    }

    s_nums.iter().sum::<u64>()
}

#[test]
fn test_pe719() {
    assert_eq!(pe719(10u64.pow(4)), 41333);
}

fn main() {
    let start = std::time::Instant::now();
    println!("Solution: {}", pe719(10u64.pow(12)));
    println!("Time elapsed: {}", start.elapsed().as_secs_f64());
}
