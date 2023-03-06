// A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.

// Find the largest palindrome made from the product of two 3-digit numbers.

fn main() {
    let largest = find_largest_palindrome(3);
    println!("The largest palindrome of a 3 digit number is: {}", largest);
}

fn find_largest_palindrome(digit_len: u32) -> u32 {
    let mut largest = 0;
    let nums = get_candidates(digit_len);
    for (x, y) in nums {
        let product = x * y;
        if is_palindrome(product) && product > largest {
            largest = product
        }
    }
    largest
}

#[test]
fn test_find_largest_palindrome() {
    assert_eq!(find_largest_palindrome(1), 9);
    assert_eq!(find_largest_palindrome(2), 9009);
}

fn get_candidates(limit: u32) -> Vec<(u32, u32)> {
    let mut nums = Vec::new();
    for x in (10 as u32).pow(limit - 1)..(10 as u32).pow(limit as u32) {
        for y in (10 as u32).pow(limit - 1)..(10 as u32).pow(limit as u32) {
            nums.push((x, y));
        }
    }
    return nums;
}

#[test]
fn test_get_candidates() {
    assert_eq!(get_candidates(1)[0], (1, 1));
    let small_vec = get_candidates(1);
    assert_eq!(small_vec[small_vec.len() - 1], (9, 9));
    assert_eq!(get_candidates(2)[0], (10, 10));
    let large_vec = get_candidates(2);
    assert_eq!(large_vec[large_vec.len() - 1], (99, 99));
}

fn is_palindrome(num: u32) -> bool {
    let num_as_str = num.to_string();
    let reversed = num_as_str.chars().rev().collect::<String>();

    num_as_str == reversed
}

#[test]
fn test_is_palindrome() {
    assert_eq!(is_palindrome(1), true);
    assert_eq!(is_palindrome(10), false);
    assert_eq!(is_palindrome(100), false);
    assert_eq!(is_palindrome(101), true);
    assert_eq!(is_palindrome(9009), true);
}
