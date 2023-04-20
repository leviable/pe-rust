// In the 20×20 grid below, four numbers along a diagonal line have been marked in red.

// 08 02 22 97 38 15 00 40 00 75 04 05 07 78 52 12 50 77 91 08
// 49 49 99 40 17 81 18 57 60 87 17 40 98 43 69 48 04 56 62 00
// 81 49 31 73 55 79 14 29 93 71 40 67 53 88 30 03 49 13 36 65
// 52 70 95 23 04 60 11 42 69 24 68 56 01 32 56 71 37 02 36 91
// 22 31 16 71 51 67 63 89 41 92 36 54 22 40 40 28 66 33 13 80
// 24 47 32 60 99 03 45 02 44 75 33 53 78 36 84 20 35 17 12 50
// 32 98 81 28 64 23 67 10 26 38 40 67 59 54 70 66 18 38 64 70
// 67 26 20 68 02 62 12 20 95 63 94 39 63 08 40 91 66 49 94 21
// 24 55 58 05 66 73 99 26 97 17 78 78 96 83 14 88 34 89 63 72
// 21 36 23 09 75 00 76 44 20 45 35 14 00 61 33 97 34 31 33 95
// 78 17 53 28 22 75 31 67 15 94 03 80 04 62 16 14 09 53 56 92
// 16 39 05 42 96 35 31 47 55 58 88 24 00 17 54 24 36 29 85 57
// 86 56 00 48 35 71 89 07 05 44 44 37 44 60 21 58 51 54 17 58
// 19 80 81 68 05 94 47 69 28 73 92 13 86 52 17 77 04 89 55 40
// 04 52 08 83 97 35 99 16 07 97 57 32 16 26 26 79 33 27 98 66
// 88 36 68 87 57 62 20 72 03 46 33 67 46 55 12 32 63 93 53 69
// 04 42 16 73 38 25 39 11 24 94 72 18 08 46 29 32 40 62 76 36
// 20 69 36 41 72 30 23 88 34 62 99 69 82 67 59 85 74 04 36 16
// 20 73 35 29 78 31 90 01 74 31 49 71 48 86 81 16 23 57 05 54
// 01 70 54 71 83 51 54 69 16 92 33 48 61 43 52 01 89 19 67 48

// The product of these numbers is 26 × 63 × 78 × 14 = 1788696.

// What is the greatest product of four adjacent numbers in the same direction (up, down, left, right, or diagonally) in the 20×20 grid?

#[derive(Clone)]
struct SmallGrid {
    rows: Vec<Vec<usize>>,
}

impl SmallGrid {
    fn new(r1: &[usize], r2: &[usize], r3: &[usize], r4: &[usize]) -> SmallGrid {
        // let mut grid = SmallGrid { rows: vec![] };
        // for row in vec![r1, r2, r3, r4] {
        //     grid.rows
        //         .push(row.iter().map(|x| x.parse().unwrap()).collect());
        // }
        // grid

        SmallGrid {
            rows: vec![r1.to_vec(), r2.to_vec(), r3.to_vec(), r4.to_vec()],
        }
    }

    fn largest_product(&self) -> usize {
        *vec![
            self.largest_row_product(),
            self.largest_col_product(),
            self.largest_diag_product(),
        ]
        .iter()
        .max()
        .unwrap()
    }

    fn largest_row_product(&self) -> usize {
        let mut largest_product = 0;
        for row in &self.rows {
            // let sum = init.iter().copied().sum::<u32>();
            let product = row.iter().product::<usize>();
            if product > largest_product {
                largest_product = product;
            }
        }

        largest_product
    }

    fn largest_col_product(&self) -> usize {
        let mut largest_product = 0;
        for col_idx in 0..self.rows.len() {
            let mut col_product = 1;
            for row in &self.rows {
                col_product *= row[col_idx];
            }
            if col_product > largest_product {
                largest_product = col_product;
            }
        }

        largest_product
    }

    fn largest_diag_product(&self) -> usize {
        let mut forward_diag_product = 1;
        let mut reverse_diag_product = 1;

        let row_len = self.rows[0].len() as usize;

        for (col_idx, row) in self.rows.iter().enumerate() {
            forward_diag_product *= row[col_idx];
            reverse_diag_product *= row[(row_len - 1) - col_idx];
        }

        *vec![forward_diag_product, reverse_diag_product]
            .iter()
            .max()
            .unwrap()
    }
}

#[test]
fn test_small_grid_new() {
    let grid = SmallGrid::new(
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
        vec![9, 10, 11, 12],
        vec![13, 14, 15, 16],
    );
    assert_eq!(grid.rows[0].len(), 4);
    assert_eq!(grid.rows[1].len(), 4);
    assert_eq!(grid.rows[2].len(), 4);
    assert_eq!(grid.rows[3].len(), 4);
    assert_eq!(grid.rows[0][0], 1);
    assert_eq!(grid.rows[1][1], 6);
    assert_eq!(grid.rows[2][2], 11);
    assert_eq!(grid.rows[3][3], 16);
}

#[test]
fn test_small_grid_largest_product() {
    let grid = SmallGrid::new(
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
        vec![9, 10, 11, 12],
        vec![13, 14, 15, 16],
    );
    assert_eq!(grid.largest_product(), 13 * 14 * 15 * 16);
}

#[test]
fn test_small_grid_largest_row_product() {
    let grid = SmallGrid::new(
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
        vec![9, 10, 11, 12],
        vec![13, 14, 15, 16],
    );
    assert_eq!(grid.largest_row_product(), 13 * 14 * 15 * 16);
}

#[test]
fn test_small_grid_largest_col_product() {
    let grid = SmallGrid::new(
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
        vec![9, 10, 11, 12],
        vec![13, 14, 15, 16],
    );
    assert_eq!(grid.largest_col_product(), 4 * 8 * 12 * 16);
}

#[test]
fn test_small_grid_largest_diag_product() {
    let grid = SmallGrid::new(
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
        vec![9, 10, 11, 12],
        vec![13, 14, 15, 16],
    );
    assert_eq!(grid.largest_diag_product(), 13 * 10 * 7 * 4);
}

fn chunk_grid(grid: &'static [&'static str]) -> Vec<SmallGrid> {
    let mut grids: Vec<SmallGrid> = vec![];
    let mut parsed_grid: Vec<Vec<usize>> = vec![];
    for row in grid {
        parsed_grid.push(row.split(" ").map(|x| x.parse().unwrap()).collect());
    }

    for row_idx in 0..parsed_grid.len() - SMALL_GRID_SIZE {
        for col_idx in 0..parsed_grid[row_idx].len() - SMALL_GRID_SIZE {
            grids.push(SmallGrid::new(
                &parsed_grid[row_idx + 0][col_idx..col_idx + SMALL_GRID_SIZE],
                &parsed_grid[row_idx + 1][col_idx..col_idx + SMALL_GRID_SIZE],
                &parsed_grid[row_idx + 2][col_idx..col_idx + SMALL_GRID_SIZE],
                &parsed_grid[row_idx + 3][col_idx..col_idx + SMALL_GRID_SIZE],
            ));
        }
    }

    grids
}

fn main() {
    let mut largest_product = 0;
    for grid in chunk_grid(GRID) {
        let largest_grid_product = grid.largest_product();
        if largest_grid_product > largest_product {
            largest_product = largest_grid_product;
        }
    }

    println!("Largest product of the grid is: {}", largest_product);
}

const SMALL_GRID_SIZE: usize = 4;
static GRID: &[&str] = &[
    "08 02 22 97 38 15 00 40 00 75 04 05 07 78 52 12 50 77 91 08",
    "49 49 99 40 17 81 18 57 60 87 17 40 98 43 69 48 04 56 62 00",
    "81 49 31 73 55 79 14 29 93 71 40 67 53 88 30 03 49 13 36 65",
    "52 70 95 23 04 60 11 42 69 24 68 56 01 32 56 71 37 02 36 91",
    "22 31 16 71 51 67 63 89 41 92 36 54 22 40 40 28 66 33 13 80",
    "24 47 32 60 99 03 45 02 44 75 33 53 78 36 84 20 35 17 12 50",
    "32 98 81 28 64 23 67 10 26 38 40 67 59 54 70 66 18 38 64 70",
    "67 26 20 68 02 62 12 20 95 63 94 39 63 08 40 91 66 49 94 21",
    "24 55 58 05 66 73 99 26 97 17 78 78 96 83 14 88 34 89 63 72",
    "21 36 23 09 75 00 76 44 20 45 35 14 00 61 33 97 34 31 33 95",
    "78 17 53 28 22 75 31 67 15 94 03 80 04 62 16 14 09 53 56 92",
    "16 39 05 42 96 35 31 47 55 58 88 24 00 17 54 24 36 29 85 57",
    "86 56 00 48 35 71 89 07 05 44 44 37 44 60 21 58 51 54 17 58",
    "19 80 81 68 05 94 47 69 28 73 92 13 86 52 17 77 04 89 55 40",
    "04 52 08 83 97 35 99 16 07 97 57 32 16 26 26 79 33 27 98 66",
    "88 36 68 87 57 62 20 72 03 46 33 67 46 55 12 32 63 93 53 69",
    "04 42 16 73 38 25 39 11 24 94 72 18 08 46 29 32 40 62 76 36",
    "20 69 36 41 72 30 23 88 34 62 99 69 82 67 59 85 74 04 36 16",
    "20 73 35 29 78 31 90 01 74 31 49 71 48 86 81 16 23 57 05 54",
    "01 70 54 71 83 51 54 69 16 92 33 48 61 43 52 01 89 19 67 48",
];
