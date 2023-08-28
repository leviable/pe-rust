// A common security method used for online banking is to ask the user for three random characters
// from a passcode. For example, if the passcode was 531278, they may ask for the 2nd, 3rd, and
// 5th characters; the expected reply would be: 317.
//
// The text file, keylog.txt, contains fifty successful login attempts.
//
// Given that the three characters are always asked for in order, analyse the file so as to determine
// the shortest possible secret passcode of unknown length.

static KEYLOG: [u64; 50] = [
    319, 680, 180, 690, 129, 620, 762, 689, 762, 318, 368, 710, 720, 710, 629, 168, 160, 689, 716,
    731, 736, 729, 316, 729, 729, 710, 769, 290, 719, 680, 318, 389, 162, 289, 162, 718, 729, 319,
    790, 680, 890, 362, 319, 760, 316, 729, 380, 319, 728, 716,
];

use std::collections::{HashMap, HashSet};

fn pe079() -> String {
    let mut map: HashMap<char, HashSet<char>> = HashMap::new();

    for partial_code in KEYLOG {
        let code_str = partial_code.to_string().chars().collect::<Vec<_>>();
        let entry_1 = &mut map.entry(code_str[0]).or_default();
        entry_1.insert(code_str[1]);
        entry_1.insert(code_str[2]);

        let entry_2 = &mut map.entry(code_str[1]).or_default();
        entry_2.insert(code_str[2]);
    }

    let mut code = String::new();

    'main_loop: for position in (0..=9).into_iter().rev() {
        for (k, v) in map.clone() {
            if v.len() == position {
                code.push(k);
                continue 'main_loop;
            }
        }

        // NOTE: this will not find the value `0`, which is the last char in the code.
    }

    code
}

fn main() {
    let start = std::time::Instant::now();

    let solution = pe079();
    println!("Solution: {solution}");
    println!("Time elapsed: {}", start.elapsed().as_secs_f64());
}
