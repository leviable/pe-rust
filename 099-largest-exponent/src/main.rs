// Comparing two numbers written in index form like
//  and
//  is not difficult, as any calculator would confirm that
// .
//
// However, confirming that
//  would be much more difficult, as both numbers contain over three million digits.
//
// Using base_exp.txt (right click and 'Save Link/Target As...'), a 22K text file containing one thousand lines with a base/exponent pair on each line, determine which line number has the greatest numerical value.
//
// NOTE: The first two lines in the file represent the numbers in the example given above.

use std::fs;
use std::io::BufRead;

#[derive(Debug)]
struct ScientificNotation {
    position: u64,
    base: f64,
    exp: f64,
    _orig_base: u64,
    _orig_exp: u64,
}

impl ScientificNotation {
    fn convert(position: u64, base: u64, exp: u64) -> Self {
        let num = (base as f64).log10() * exp as f64;

        let e = num.floor();
        let m = 10_f64.powf(num - e);

        Self {
            position,
            base: m as f64,
            exp: e,
            _orig_base: base,
            _orig_exp: exp,
        }
    }
}

fn pe099() -> u64 {
    let mut nums: Vec<ScientificNotation> = vec![];
    let file = fs::File::open("base_exp.txt").unwrap();
    for (idx, line) in std::io::BufReader::new(file).lines().enumerate() {
        let line = line.expect("failed to read the line");

        let num = line
            .split(",")
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        nums.push(ScientificNotation::convert(idx as u64, num[0], num[1]));
    }

    nums.sort_by(|a, b| match a.exp.total_cmp(&b.exp) {
        std::cmp::Ordering::Less => std::cmp::Ordering::Less,
        std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
        std::cmp::Ordering::Equal => match a.base.total_cmp(&b.base) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
            std::cmp::Ordering::Equal => {
                unreachable!();
            }
        },
    });

    nums.iter().next_back().unwrap().position + 1
}

fn main() {
    let start = std::time::Instant::now();
    let solution = pe099();
    println!("Solution: {}", solution);
    println!("Time elapsed: {}", start.elapsed().as_secs_f64());
}
