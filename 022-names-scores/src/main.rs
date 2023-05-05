// Using names.txt (right click and 'Save Link/Target As...'), a 46K text file
// containing over five-thousand first names, begin by sorting it into alphabetical
// order. Then working out the alphabetical value for each name, multiply this value
// by its alphabetical position in the list to obtain a name score.

// For example, when the list is sorted into alphabetical order, COLIN, which is
// worth 3 + 15 + 12 + 9 + 14 = 53, is the 938th name in the list. So, COLIN would
// obtain a score of 938 × 53 = 49714.

// What is the total of all the name scores in the file?

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::time::Instant;

fn get_name_score(name: &str) -> usize {
    let mut score = 0;
    for letter in name.chars() {
        score += letter as usize - 64;
    }
    score
}

#[test]
fn test_get_name_score() {
    assert_eq!(get_name_score("COLIN"), 53);
}

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    let file = File::open(filename).unwrap();
    return io::BufReader::new(file).lines();
}

fn main() {
    let start = Instant::now();
    // Load in file
    let mut names: Vec<String> = vec![];
    for line in read_lines("./src/names.txt".to_string()) {
        for name in line.expect("REASON").split(",") {
            names.push(name.replace("\"", ""));
        }
    }
    names.sort();

    let mut total: usize = 0;

    for (idx, name) in names.iter().enumerate() {
        // println!("{idx} {name}");
        let score = get_name_score(name);
        total += score * (idx + 1);
    }
    println!("Total score: {}", total);
    println!("Elapsed time: {}", start.elapsed().as_secs_f64());
}
