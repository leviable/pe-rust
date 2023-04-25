// If the numbers 1 to 5 are written out in words: one, two, three, four, five, then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.

// If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words, how many letters would be used?

// NOTE: Do not count spaces or hyphens. For example, 342 (three hundred and
//       forty-two) contains 23 letters and 115 (one hundred and fifteen) contains
//       20 letters. The use of "and" when writing out numbers is in compliance
//       with British usage.

use std::collections::HashMap;

fn to_words(mut num: usize) -> String {
    let mut words: Vec<&str> = Vec::new();
    let mut hashmap: HashMap<usize, &str> = HashMap::new();
    hashmap.insert(1, "one");
    hashmap.insert(2, "two");
    hashmap.insert(3, "three");
    hashmap.insert(4, "four");
    hashmap.insert(5, "five");
    hashmap.insert(6, "six");
    hashmap.insert(7, "seven");
    hashmap.insert(8, "eight");
    hashmap.insert(9, "nine");
    hashmap.insert(10, "ten");
    hashmap.insert(11, "eleven");
    hashmap.insert(12, "twelve");
    hashmap.insert(13, "thirteen");
    hashmap.insert(14, "fourteen");
    hashmap.insert(15, "fifteen");
    hashmap.insert(16, "sixteen");
    hashmap.insert(17, "seventeen");
    hashmap.insert(18, "eighteen");
    hashmap.insert(19, "nineteen");
    hashmap.insert(20, "twenty");
    hashmap.insert(30, "thirty");
    hashmap.insert(40, "forty");
    hashmap.insert(50, "fifty");
    hashmap.insert(60, "sixty");
    hashmap.insert(70, "seventy");
    hashmap.insert(80, "eighty");
    hashmap.insert(90, "ninety");

    if num > 999 {
        words.push("one");
        words.push("thousand");
        num = num % 1000;
    }

    if num > 99 {
        let hundo = num / 100;
        num = num % 100;
        words.push(hashmap.get(&hundo).unwrap());
        words.push("hundred");

        if num > 0 {
            words.push("and");
        }
    }

    if num > 9 && num < 20 {
        words.push(hashmap.get(&num).unwrap());
    } else if num >= 20 {
        let tens = num / 10 * 10;
        num = num % 10;
        words.push(hashmap.get(&tens).unwrap());

        if num != 0 {
            words.push(hashmap.get(&num).unwrap());
        }
    } else if num > 0 && num < 10 {
        words.push(hashmap.get(&num).unwrap());
    }

    words.join("")
}

#[test]
fn test_to_words() {
    assert_eq!(to_words(1), "one");
    assert_eq!(to_words(1000), "onethousand");
    assert_eq!(to_words(311), "threehundredandeleven");
    assert_eq!(to_words(342), "threehundredandfortytwo");
    assert_eq!(to_words(10), "ten");
    assert_eq!(to_words(11), "eleven");
    assert_eq!(to_words(12), "twelve");
    assert_eq!(to_words(666), "sixhundredandsixtysix");
    assert_eq!(to_words(55), "fiftyfive");
}

fn main() {
    let mut total = 0;

    for num in 1..=1000 {
        let words = to_words(num);
        total += words.len();
    }

    println!("Total letter count: {}", total);
}
