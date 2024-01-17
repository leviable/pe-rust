// In the 5 by 5 matrix below, the minimal path sum from the top left
// to the bottom right, by only moving to the right and down, is
// indicated in bold red and is equal to 2427.
//
// 131 673 234 103  18
// 201  96 342 965 150
// 630 803 746 422 111
// 537 699 497 121 956
// 805 732 524  37 331
//
// Find the minimal path sum from the top left to the bottom right
// by only moving right and down in matrix.txt (right click and
// "Save Link/Target As..."), a 31K text file containing an 80 by 80
// matrix.

use std::fs;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Clone)]
struct Matrix {
    data: Vec<Vec<usize>>,
    end: (usize, usize),
}

impl Matrix {
    fn load(filename: &str) -> Self {
        let buf = BufReader::new(fs::File::open(filename).unwrap());
        let data = buf
            .lines()
            .map(|line| {
                line.unwrap()
                    .split(",")
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let end = (data.len() - 1, data[data.len() - 1].len() - 1);

        Self { data, end }
    }

    fn cell(self, x: usize, y: usize) -> usize {
        self.data[y][x]
    }

    fn shortest_path(&self, x: usize, y: usize) -> Vec<usize> {
        if x == 0 && y == 0 {
            let cell_val = self.clone().cell(x, y);
            return vec![cell_val];
        }

        if x == 0 {
            let mut next = self.shortest_path(x, y - 1);
            next.push(self.clone().cell(x, y));
            next
        } else if y == 0 {
            let mut next = self.shortest_path(x - 1, y);
            next.push(self.clone().cell(x, y));
            next
        } else {
            let above = self.clone().cell(x, y.saturating_sub(1));
            let left = self.clone().cell(x.saturating_sub(1), y);

            let next = match above.cmp(&left) {
                std::cmp::Ordering::Greater => (x.saturating_sub(1), y),
                std::cmp::Ordering::Less => (x, y.saturating_sub(1)),
                std::cmp::Ordering::Equal => unreachable!("Adjacent cells are never equal"),
            };

            let mut next = self.shortest_path(next.0, next.1);

            next.push(self.clone().cell(x, y));
            next
        }
    }
}

#[test]
fn test_shortest_path() {
    let m = Matrix::load("matrix-example.txt");
    let matrix_end = (4, 4);
    assert_eq!(
        m.shortest_path(matrix_end.0, matrix_end.1),
        vec![131, 201, 96, 342, 746, 422, 121, 37, 331]
    );
}

fn pe081(matrix_file: &str) -> usize {
    let matrix = Matrix::load(matrix_file);

    let binding = matrix.shortest_path(matrix.end.0, matrix.end.1);
    eprintln!("{:?}", binding);
    let binding = binding.iter();
    binding.sum()
}

#[test]
fn test_pe081() {
    assert_eq!(pe081("matrix-example.txt"), 2427);
}

fn main() {
    let start = std::time::Instant::now();
    let solution = pe081("0081_matrix.txt");
    println!("Solution: {solution}");
    println!("Elapsed time: {}", start.elapsed().as_secs_f64());
}
