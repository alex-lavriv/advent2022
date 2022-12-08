use std::io::prelude::*;
use std::fs::File;

fn main() {
    let vec = parse_input();
    let ans = solve(&vec);
    println!("The ans is {ans}")
}

fn solve(vec: &Vec<Vec<u8>>) -> usize {
    let mut ans = (vec.len() + vec[0].len()) * 2 - 4;

    for i in 1..vec.len() - 1 {
        for j in 1..vec[i].len() - 1 {
            if is_visible(vec, i, j) {
                ans += 1;
            }
        }
    }
    ans
}

fn is_visible(vec: &Vec<Vec<u8>>, x: usize, y: usize) -> bool {
    let val = vec[x][y];
    let mut v_visible = true;
    let mut h_visible = true;
    for i in 0..vec.len() {
        if vec[i][y] >= val && i != x {
            h_visible = false;
        }
        if i == x{
            h_visible = true;
        }
    }


    for i in 0..vec[x].len() {
        if vec[x][i] >= val && i != y {
            v_visible = false;
        }
        if i == y{
            v_visible = true;
        }
    }

    v_visible || h_visible
}

fn parse_input() -> Vec<Vec<u8>> {
    let input = include_str!("../8.input");
    let mut result = Vec::new();

    for line in input.lines() {
        let lineVec = line.chars().map(|c| c.to_string().parse::<u8>().unwrap()).collect::<Vec<u8>>();
        result.push(lineVec);
    }
    result
}

#[test]
fn test()
{
    let input = parse_input();
    for vec in &input {
        for c in vec {
            print!("{c} ")
        }
        println!();
    }
    assert_eq!(is_visible(&input, 3, 3), false);
}