// By starting at the top of the triangle below and moving to adjacent numbers on the row below, the maximum total from top to bottom is 23.

// 3
// 7 4
// 2 4 6
// 8 5 9 3

// That is, 3 + 7 + 4 + 9 = 23.

// Find the maximum total from top to bottom of the triangle below:

// 75
// 95 64
// 17 47 82
// 18 35 87 10
// 20 04 82 47 65
// 19 01 23 75 03 34
// 88 02 77 73 07 63 67
// 99 65 04 28 06 16 70 92
// 41 41 26 56 83 40 80 70 33
// 41 48 72 33 47 32 37 16 94 29
// 53 71 44 65 25 43 91 52 97 51 14
// 70 11 33 28 77 73 17 78 39 68 17 57
// 91 71 52 38 17 14 91 43 58 50 27 29 48
// 63 66 04 68 89 53 67 30 73 16 69 87 40 31
// 04 62 98 27 23 09 70 98 73 93 38 53 60 04 23

use std::time::Instant;

static PYRAMID: &[&str] = &[
    "75",
    "95 64",
    "17 47 82",
    "18 35 87 10",
    "20 04 82 47 65",
    "19 01 23 75 03 34",
    "88 02 77 73 07 63 67",
    "99 65 04 28 06 16 70 92",
    "41 41 26 56 83 40 80 70 33",
    "41 48 72 33 47 32 37 16 94 29",
    "53 71 44 65 25 43 91 52 97 51 14",
    "70 11 33 28 77 73 17 78 39 68 17 57",
    "91 71 52 38 17 14 91 43 58 50 27 29 48",
    "63 66 04 68 89 53 67 30 73 16 69 87 40 31",
    "04 62 98 27 23 09 70 98 73 93 38 53 60 04 23",
];

fn convert_pyramid(pyramid: &[&str]) -> Vec<Vec<usize>> {
    let mut new_pyr: Vec<Vec<usize>> = vec![];
    for row in pyramid.iter().rev() {
        new_pyr.push(row.split(" ").map(|x| x.parse().unwrap()).collect());
    }
    new_pyr
}

fn get_max_path(pyramid: Vec<Vec<usize>>, index: usize) -> Vec<usize> {
    if pyramid.len() == 1 {
        return vec![pyramid[0][0]];
    }

    let mut paths: Vec<Vec<usize>> = vec![];

    if index == 0 {
        let mut p = get_max_path(pyramid[1..].to_vec(), index);
        p.push(pyramid[0][index]);
        paths.push(p);
    } else if index == pyramid[0].len() - 1 {
        let mut p = get_max_path(pyramid[1..].to_vec(), index - 1);
        p.push(pyramid[0][index]);
        paths.push(p);
    } else {
        let mut p = get_max_path(pyramid[1..].to_vec(), index);
        p.push(pyramid[0][index]);
        paths.push(p);

        let mut p = get_max_path(pyramid[1..].to_vec(), index - 1);
        p.push(pyramid[0][index]);
        paths.push(p);
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
    let TEST_PYRAMID: &[&str] = &["3", "7 4", "2 4 6", "8 5 9 3"];
    let p = convert_pyramid(TEST_PYRAMID);
    assert_eq!(get_max_path(p.clone(), 0), vec!(3, 7, 2, 8));
    assert_eq!(get_max_path(p.clone(), 1), vec!(3, 7, 4, 5));
    assert_eq!(get_max_path(p.clone(), 2), vec!(3, 7, 4, 9));
    assert_eq!(get_max_path(p.clone(), 3), vec!(3, 4, 6, 3));
}

#[test]
fn test_convert_pyramid() {
    let TEST_PYRAMID: &[&str] = &["8 5 9 3", "2 4 6", "7 4", "3"];

    let expected: Vec<Vec<usize>> = vec![vec![3], vec![7, 4], vec![2, 4, 6], vec![8, 5, 9, 3]];

    assert_eq!(convert_pyramid(TEST_PYRAMID), expected);
}

fn main() {
    let start = Instant::now();

    let p = convert_pyramid(PYRAMID);

    let mut max_paths: Vec<Vec<usize>> = vec![];
    let mut max_path_sum = 0;
    for (index, _) in p.clone().iter().enumerate() {
        max_paths.push(get_max_path(p.clone(), index));
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
