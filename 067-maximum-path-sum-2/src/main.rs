// By starting at the top of the triangle below and moving to adjacent numbers on the row
// below, the maximum total from top to bottom is 23.

// 3
// 7 4
// 2 4 6
// 8 5 9 3

// That is, 3 + 7 + 4 + 9 = 23.

// Find the maximum total from top to bottom in triangle.txt (right click and 'Save
// Link/Target As...'), a 15K text file containing a triangle with one-hundred rows.

// NOTE: This is a much more difficult version of Problem 18. It is not possible to try
//       every route to solve this problem, as there are 299 altogether! If you could
//       check one trillion (1012) routes every second it would take over twenty billion
//       years to check them all. There is an efficient algorithm to solve it. ;o)

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::time::Instant;

fn convert_pyramid(pyramid: Vec<String>) -> Vec<Vec<usize>> {
    let mut new_pyr: Vec<Vec<usize>> = vec![];
    for row in pyramid.iter().rev() {
        new_pyr.push(row.split(" ").map(|x| x.parse().unwrap()).collect());
    }
    new_pyr
}

fn get_max_path(
    hm: &mut HashMap<(usize, usize), Vec<usize>>,
    pyramid: Vec<Vec<usize>>,
    row: usize,
    index: usize,
) -> Vec<usize> {
    if pyramid.len() == 1 {
        return vec![pyramid[0][0]];
    }

    let mut paths: Vec<Vec<usize>> = vec![];
    let mut next: Vec<usize> = vec![];

    if index == 0 {
        next.push(index);
    } else if index == pyramid[0].len() - 1 {
        next.push(index - 1);
    } else {
        next.push(index);
        next.push(index - 1);
    }

    for n in next {
        if !hm.contains_key(&(row, n)) {
            let p = get_max_path(hm, pyramid[1..].to_vec(), row + 1, n);
            hm.insert((row, n), p.clone());
        }

        let mut path: Vec<usize> = hm.get(&(row, n)).unwrap().clone();
        path.push(pyramid[0][index]);
        paths.push(path);
    }

    let mut max = 0;
    let mut max_path: Vec<usize> = vec![];
    for p in paths {
        let path_val = p.iter().fold(0, |acc, x| acc + x);
        if path_val > max {
            max = path_val;
            max_path = p;
        }
    }
    max_path
}

#[test]
fn test_get_max_path() {
    let TEST_PYRAMID: Vec<String> = vec![
        "3".to_string(),
        "7 4".to_string(),
        "2 4 6".to_string(),
        "8 5 9 3".to_string(),
    ];
    let mut hm: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    let p = convert_pyramid(TEST_PYRAMID);
    assert_eq!(get_max_path(&mut hm, p.clone(), 0, 0), vec!(3, 7, 2, 8));
    assert_eq!(get_max_path(&mut hm, p.clone(), 0, 1), vec!(3, 7, 4, 5));
    assert_eq!(get_max_path(&mut hm, p.clone(), 0, 2), vec!(3, 7, 4, 9));
    assert_eq!(get_max_path(&mut hm, p.clone(), 0, 3), vec!(3, 4, 6, 3));
}

#[test]
fn test_convert_pyramid() {
    let TEST_PYRAMID: Vec<String> = vec![
        "8 5 9 3".to_string(),
        "2 4 6".to_string(),
        "7 4".to_string(),
        "3".to_string(),
    ];

    let expected: Vec<Vec<usize>> = vec![vec![3], vec![7, 4], vec![2, 4, 6], vec![8, 5, 9, 3]];

    assert_eq!(convert_pyramid(TEST_PYRAMID), expected);
}

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    let file = File::open(filename).unwrap();
    return io::BufReader::new(file).lines();
}

fn main() {
    let start = Instant::now();

    let mut hm: HashMap<(usize, usize), Vec<usize>> = HashMap::new();

    let mut lines: Vec<String> = vec![];
    for line in read_lines("./src/triangle.txt".to_string()) {
        lines.push(line.unwrap());
    }

    let p = convert_pyramid(lines);

    let mut max_paths: Vec<Vec<usize>> = vec![];
    let mut max_path_sum = 0;
    for (index, _) in p.clone().iter().enumerate() {
        max_paths.push(get_max_path(&mut hm, p.clone(), 0, index));
        let new_sum = max_paths[max_paths.len() - 1]
            .iter()
            .fold(0, |acc, x| acc + x);
        if new_sum > max_path_sum {
            max_path_sum = new_sum;
        }
    }

    println!("Max Path sum is {max_path_sum}");
    println!("Took {}s to complete", start.elapsed().as_secs_f64());
}
