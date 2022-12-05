use std::fs::create_dir;

fn main() {
    println!("Hello, world!");
    let crates = include_str!("../crates");
    let commands = include_str!("../commands");
    let mut crates1 = parse_crates(crates);
    let mut crates2 = parse_crates(crates);
    let mut crates = crates1;
    let commands = commands.lines().map(parse_commands);
    //print_crate(crates.clone());
    for (n, from, to) in commands.clone() {
        for _ in 0..n {
            let top = crates[from - 1].pop().unwrap();
            crates[to - 1].push(top)
        }
    }
    print_crate(crates.clone());
    crates.iter().for_each(|current| {
        print!("{}", current.last().unwrap())
    });
    print!("\n");

    let mut crates = crates2;
    for (n, from, to) in commands {
        let vec = crates[from - 1].clone();
        crates[to - 1].extend_from_slice(&vec[vec.len() - n..]);
        for _ in 0..n {
            let _ = crates[from - 1].pop().unwrap();
        }
    }
    print_crate(crates.clone());
    crates.iter().for_each(|current| {
        print!("{}", current.last().unwrap())
    });
    print!("\n");
}


fn print_crate(crates: Vec<Vec<char>>) {
    crates.iter().for_each(|current| {
        current.iter().for_each(|char| { print!("{char}") });
        print!("\n");
    })
}

fn parse_crates(crates: &str) -> Vec<Vec<char>> {
    crates.lines().map(|line: &str| {
        line.chars().rev().filter(|&c| c != '*').collect::<Vec<char>>()
    }).collect()
}

fn parse_commands(command: &str) -> (usize, usize, usize) {
    let mut parts = command.split_whitespace().map(|s| s.parse::<usize>());
    match (parts.next(), parts.next(), parts.next()) {
        (Some(Ok(a)), Some(Ok(b)), Some(Ok(c))) => {
            (a, b, c)
        }
        _ => { panic!() }
    }
}

#[test]
fn test() {
    println!("Hello, test!")
}
