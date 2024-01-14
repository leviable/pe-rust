// It is possible to write five as a sum in exactly six different ways:
//
// 4 + 1
// 3 + 2
// 3 + 1 + 1
// 2 + 2 + 1
// 2 + 1 + 1 + 1
// 1 + 1 + 1 + 1 +1
//
// How many different ways can one hundred be written as a sum of at
// least two positive integers?
//
fn find_sum(count: &mut usize, limit: usize, nums: Vec<usize>) {
    let current_sum = nums.iter().sum::<usize>();

    if current_sum == limit {
        *count += 1;
        return;
    }

    let last_num = nums.iter().next_back().unwrap();
    for num in (1..=*last_num).into_iter().rev() {
        let mut new_nums = nums.clone();
        new_nums.push(num);

        if new_nums.iter().sum::<usize>() > limit {
            continue;
        }
        find_sum(count, limit, new_nums);
    }
}

#[test]
fn test_find_sum() {
    let mut count = 0;
    find_sum(&mut count, 5, vec![4]);
    assert_eq!(count, 1);

    let mut count = 0;
    find_sum(&mut count, 5, vec![3]);
    assert_eq!(count, 2);

    let mut count = 0;
    find_sum(&mut count, 5, vec![3, 1]);
    assert_eq!(count, 1);
}

fn pe076(limit: usize) -> usize {
    let mut count = 0;

    for num in (1..limit).into_iter().rev() {
        let nums = vec![num];
        find_sum(&mut count, limit, nums);
    }

    count
}

#[test]
fn test_pe076() {
    assert_eq!(pe076(5), 6);
}

fn main() {
    let start = std::time::Instant::now();
    let solution = pe076(100);
    println!("Solution: {solution}");
    println!("Time elapsed: {}", start.elapsed().as_secs_f64());
}
