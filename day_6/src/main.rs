use std::collections::VecDeque;
use itertools::Itertools;

fn main() {
    println!("Hello, world!");
    let input = include_str!("../6.input");
    println!("{}", solve_marker(input));
}

fn solve_marker(input: &str) -> usize {
    const MARKER_SIZE: usize = 14;
    let mut set: VecDeque<char> =  Default::default();
    let chars: Vec<char> = input.chars().collect();

    set.extend(chars[0..MARKER_SIZE].iter());
    if set.clone().into_iter().unique().collect::<Vec<char>>().len() == MARKER_SIZE {
        return MARKER_SIZE;
    } else {

        for index in MARKER_SIZE..chars.len(){
            set.pop_front();
            set.push_back(chars[index]);
            if set.clone().into_iter().into_iter().unique().collect::<Vec<char>>().len() == MARKER_SIZE{
                return index + 1;
            }
        }
    }
    panic!();
}

#[test]
fn test() {
    println!("Hello, world!");
    assert_eq!(solve_marker("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
    assert_eq!(solve_marker("nppdvjthqldpwncqszvftbrmjlhg"), 6);
    assert_eq!(solve_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
    assert_eq!(solve_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
}
