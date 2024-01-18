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

use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
struct Matrix {
    data: Vec<Vec<usize>>,
    end: (usize, usize),
    hmap: HashMap<(usize, usize), Vec<usize>>,
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

        Self {
            data,
            end,
            hmap: HashMap::new(),
        }
    }

    fn cell(self, x: usize, y: usize) -> usize {
        self.data[y][x]
    }

    fn shortest_path(&mut self, x: usize, y: usize) -> Vec<usize> {
        if !self.hmap.contains_key(&(x, y)) {
            if x == 0 && y == 0 {
                let cell_val = self.clone().cell(x, y);
                let to_insert = vec![cell_val];
                self.hmap.insert((x, y), to_insert);
            } else if x == 0 {
                let mut next = self.shortest_path(x, y - 1);
                next.push(self.clone().cell(x, y));
                let to_insert = next;
                self.hmap.insert((x, y), to_insert);
            } else if y == 0 {
                let mut next = self.shortest_path(x - 1, y);
                next.push(self.clone().cell(x, y));
                let to_insert = next;
                self.hmap.insert((x, y), to_insert);
            } else {
                let mut above = self.shortest_path(x, y.saturating_sub(1));
                above.push(self.clone().cell(x, y));

                let mut left = self.shortest_path(x.saturating_sub(1), y);
                left.push(self.clone().cell(x, y));

                let next = match above.iter().sum::<usize>().cmp(&left.iter().sum::<usize>()) {
                    std::cmp::Ordering::Greater => left,
                    std::cmp::Ordering::Less => above,
                    std::cmp::Ordering::Equal => unreachable!("Adjacent cells are never equal"),
                };

                let to_insert = next;
                self.hmap.insert((x, y), to_insert);
            }
        }

        let path = self.hmap.get(&(x, y)).unwrap();
        path.clone()
    }
}

#[test]
fn test_shortest_path() {
    let mut m = Matrix::load("matrix-example.txt");
    let matrix_end = (4, 4);
    assert_eq!(
        m.shortest_path(matrix_end.0, matrix_end.1),
        vec![131, 201, 96, 342, 746, 422, 121, 37, 331]
    );
}

fn pe081(matrix_file: &str) -> usize {
    let mut matrix = Matrix::load(matrix_file);

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
