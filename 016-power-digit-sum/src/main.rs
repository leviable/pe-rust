// 2**15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.

// What is the sum of the digits of the number 2**1000?

use std::fmt;

struct Two {
    num: Vec<usize>,
}

impl Two {
    fn new() -> Two {
        let num = vec![2];
        Two { num: num }
    }

    fn power(&mut self, pow: usize) {
        let mut count = 1;
        while count < pow {
            let mut new_num: Vec<usize> = vec![];
            let mut remainder = 0;
            for num in &self.num {
                new_num.push(((num * 2) + remainder) % 10);
                remainder = ((num * 2) + remainder) / 10;
            }
            while remainder > 0 {
                new_num.push(remainder % 10);
                remainder /= 10;
            }
            self.num = new_num;
            count += 1;
        }
    }

    fn summed(&self) -> String {
        self.num
            .iter()
            .fold(0, |sum, digit| sum + digit)
            .to_string()
    }
}

impl fmt::Display for Two {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.num
                .iter()
                .rev()
                .map(|x| { x.to_string() })
                .collect::<String>()
        )
    }
}

#[test]
fn test_big_num_new() {
    let b = Two::new();
    assert_eq!(b.num, vec![2]);
}

#[test]
fn test_big_num_power() {
    let mut b = Two::new();
    b.power(4);
    assert_eq!(b.num, vec![6, 1]);
    let mut c = Two::new();
    c.power(15);
    assert_eq!(c.num, vec![8, 6, 7, 2, 3]);
}

fn main() {
    let mut num = Two::new();
    num.power(1000);
    println!("2**1000 is: {}", num);
    println!("2**1000 summed is: {}", num.summed());
}
