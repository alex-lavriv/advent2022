#![feature(int_roundings)]

use std::collections::VecDeque;


fn main() {
    println!("Hello, world!");
    let mut monkeys = init();
    for round in 0..10000 * monkeys.len() {
        let current_monkey = round % monkeys.len();
        for (item, monkey_n) in monkeys[current_monkey].throw() {
            monkeys[monkey_n].items.push_back(item);
        }
    }
    for monkey in &monkeys {
        println!("Monkey inspect count {}", monkey.inspect_count);
    }
    monkeys.sort_by(|a, b| b.inspect_count.cmp(&a.inspect_count));

    println!("The ans is {}", monkeys[0].inspect_count * monkeys[1].inspect_count)
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Monkey {
    pub items: VecDeque<u128>,
    operation: fn(u128) -> u128,
    divisible_by_test: u128,
    if_test_true: usize,
    if_test_false: usize,
    pub inspect_count: usize,
}


impl Monkey {
    pub fn throw(&mut self) -> Vec<(u128, usize)> {
        let mut result = Vec::new();
        for &item in &self.items {
            let mut item_div = item % (7 * 5 * 11 * 2 * 3 * 19 * 13 * 17 * 23);

            let level = (self.operation)(item_div);
            if level % self.divisible_by_test == 0 {
                result.push((level, self.if_test_true));
            } else {
                result.push((level, self.if_test_false));
            }
        }
        self.items.clear();
        self.inspect_count += result.len();
        result
    }
}

fn init() -> Vec<Monkey> {
    let m0 = Monkey { items: VecDeque::from_iter([64, 89, 65, 95]), operation: |old| old * 7, divisible_by_test: 3, if_test_true: 4, if_test_false: 1, inspect_count: 0 };
    let m1 = Monkey { items: VecDeque::from_iter([76, 66, 74, 87, 70, 56, 51, 66]), operation: |old| old + 5, divisible_by_test: 13, if_test_true: 7, if_test_false: 3, inspect_count: 0 };
    let m2 = Monkey { items: VecDeque::from_iter([91, 60, 63]), operation: |old| old * old, divisible_by_test: 2, if_test_true: 6, if_test_false: 5, inspect_count: 0 };
    let m3 = Monkey { items: VecDeque::from_iter([92, 61, 79, 97, 79]), operation: |old| old + 6, divisible_by_test: 11, if_test_true: 2, if_test_false: 6, inspect_count: 0 };
    let m4 = Monkey { items: VecDeque::from_iter([93, 54]), operation: |old| old * 11, divisible_by_test: 5, if_test_true: 1, if_test_false: 7, inspect_count: 0 };
    let m5 = Monkey { items: VecDeque::from_iter([60, 79, 92, 69, 88, 82, 70]), operation: |old| old + 8, divisible_by_test: 17, if_test_true: 4, if_test_false: 0, inspect_count: 0 };
    let m6 = Monkey { items: VecDeque::from_iter([64, 57, 73, 89, 55, 53]), operation: |old| old + 1, divisible_by_test: 19, if_test_true: 0, if_test_false: 5, inspect_count: 0 };
    let m7 = Monkey { items: VecDeque::from_iter([62]), operation: |old| old + 4, divisible_by_test: 7, if_test_true: 3, if_test_false: 2, inspect_count: 0 };
    vec![m0, m1, m2, m3, m4, m5, m6, m7]
}

fn init_example() -> Vec<Monkey> {
    let m0 = Monkey { items: VecDeque::from_iter([79, 98]), operation: |old| old * 19, divisible_by_test: 23, if_test_true: 2, if_test_false: 3, inspect_count: 0 };
    let m1 = Monkey { items: VecDeque::from_iter([54, 65, 75, 74]), operation: |old| old + 6, divisible_by_test: 19, if_test_true: 2, if_test_false: 0, inspect_count: 0 };
    let m2 = Monkey { items: VecDeque::from_iter([79, 60, 97]), operation: |old| old * old, divisible_by_test: 13, if_test_true: 1, if_test_false: 3, inspect_count: 0 };
    let m3 = Monkey { items: VecDeque::from_iter([74]), operation: |old| old + 3, divisible_by_test: 17, if_test_true: 0, if_test_false: 1, inspect_count: 0 };
    vec![m0, m1, m2, m3]
}
