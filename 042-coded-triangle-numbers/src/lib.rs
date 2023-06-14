// pub mod pe042;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

// This is obviously the slowest possible way to do this, recalculating
// triangle numbers for each word. Should be smarter by storing them in
// a HashMap, and then adding only when I find a number I don't already
// have
// TODO: incorporate a HashMap
pub fn is_triangle_number(num: usize) -> bool {
    let num = num as f64;
    let mut x = 0.0;
    loop {
        x += 1.0;
        match (x * 0.5) * (x + 1.0) {
            f if f == num => return true,
            f if f < num => continue,
            f if f > num => return false,
            _ => unreachable!(),
        };
    }
}

pub fn calc_word_score(word: &String) -> usize {
    word.chars().map(|x| x as usize - 64).sum()
}

pub fn count_triangle_words(word_buf: &Vec<String>) -> usize {
    let mut total: usize = 0;
    for word in word_buf {
        let word_score = calc_word_score(&word);
        if is_triangle_number(word_score) {
            // println!("Found one with score of {}: {}", word_score, word);
            total += 1;
        }
    }
    total
}

pub fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    let file = File::open(filename).unwrap();
    return io::BufReader::new(file).lines();
}
