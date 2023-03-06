// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.

// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

fn main() {
    println!(
        "Smallest evenly divisible from 1 to 20 is: {}",
        find_smallest_positive(20)
    );
}

fn find_smallest_positive(upper_bound: u32) -> u32 {
    let nums = get_nums(upper_bound);
    let from_num = nums[nums.len() - 1];
    let mut smallest = 0;
    'outer: for val in (from_num..).step_by(from_num as usize) {
        // println!("{:?} {}", nums, val);
        for num in nums.clone() {
            if val % num != 0 {
                continue 'outer;
            }
        }
        smallest = val;
        break;
    }

    smallest
}

#[test]
fn test_find_smallest_positive() {
    assert_eq!(find_smallest_positive(1), 1);
    assert_eq!(find_smallest_positive(2), 2);
    assert_eq!(find_smallest_positive(3), 6);
    assert_eq!(find_smallest_positive(10), 2520);
}

fn get_nums(upper_bound: u32) -> Vec<u32> {
    (1..upper_bound + 1).collect()
}

#[test]
fn test_get_nums() {
    assert_eq!(get_nums(1), vec![1]);
    assert_eq!(get_nums(2), vec![1, 2]);
    assert_eq!(get_nums(3), vec![1, 2, 3]);
}
