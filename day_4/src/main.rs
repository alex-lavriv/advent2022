use std::ffi::c_int;

fn main() {
    println!("Hello, world!");

    let input = include_str!("../4.input");

    let count = input.lines().
        filter(|&line| {
            let split: Vec<&str> = line.split(",").collect();
            let (part1, part2) = (split[0], split[1]);
            contains(parse_line(part1), parse_line(part2))
        }).count();
    println!("{count}");

    let count = input.lines().
        filter(|&line| {
            let split: Vec<&str> = line.split(",").collect();
            let (part1, part2) = (split[0], split[1]);
            overlap(parse_line(part1), parse_line(part2)) ||
                overlap(parse_line(part2), parse_line(part1))
        }).count();
    println!("{count}");
}

fn parse_line(line: &str) -> (u32, u32) {
    let mut parts = line.split("-").map(|s| s.parse::<u32>());
    match (parts.next(), parts.next()) {
        (Some(Ok(a)), Some(Ok(b))) => {
            (a, b)
        }
        _ => { panic!() }
    }
}

fn contains(section1: (u32, u32), section2: (u32, u32)) -> bool {
    let (a1, b1) = section1;
    let (a2, b2) = section2;

    (a2 <= a1) && (b1 <= b2) || (a1 <= a2) && (b2 <= b1)
}

fn overlap(section1: (u32, u32), section2: (u32, u32)) -> bool {
    let (a1, _) = section1;
    let (a2, b2) = section2;

    a2 <= a1 && a1 <= b2
}

#[test]
fn test() {
    println!("Test");
    assert_eq!(parse_line("2-6"), (2, 6))
}