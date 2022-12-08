use std::io::prelude::*;
use std::fs::File;

fn main() {
    let vec = parse_input();
    let ans = solve(&vec);

    println!("The ans is {ans}");
    let max_view = get_score(&vec);
    println!("The max_view is {max_view}");
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

fn get_score(vec: &Vec<Vec<u8>>) -> u32 {
    let mut max = 0;
    for i in 1..vec.len() - 1 {
        for j in 1..vec[i].len() - 1 {
            let m = is_score_single(vec, i, j);
            if (max < m){
                max = m;
            }
        }
    }
    max
}

fn is_score_single(vec: &Vec<Vec<u8>>, x: usize, y: usize) -> u32 {
    let val = vec[x][y];
    let mut score = 1;
    let mut count = 0;
    //left
    for i in (0..=(y - 1)).rev() {
        let entry = vec[x][i];
        if entry >= val {

            count += 1;
            break;
        } else {
            count += 1;
        }
    }
    score *= count;
    count = 0;
    //right
    for i in y + 1..vec.len() {
        let entry = vec[x][i];
        if entry >= val {
            count += 1;
            break;
        } else {
            count += 1;
        }
    }
    score *= count;
    count = 0;
    //top
    for i in (0..=(x - 1)).rev() {
        let entry = vec[i][y];
        if entry >= val {
            count += 1;
            break;
        } else {
            count += 1;
        }
    }
    score *= count;
    count = 0;
    //bottom
    for i in (x + 1)..vec[x].len() {
        let entry = vec[i][y];
        if entry >= val {
            count += 1;
            break;
        } else {
            count += 1;
        }
    }
    score *= count;


    score
}

fn is_visible(vec: &Vec<Vec<u8>>, x: usize, y: usize) -> bool {
    let val = vec[x][y];
    let mut v_visible = true;
    let mut h_visible = true;
    for i in 0..vec.len() {
        if vec[i][y] >= val && i != x {
            h_visible = false;
        }
        if i == x {
            if h_visible { return true; }
            h_visible = true;
        }
    }

    for i in 0..vec[x].len() {
        if vec[x][i] >= val && i != y {
            v_visible = false;
        }
        if i == y {
            if v_visible { return true; }
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
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            print!(" ({},{}) {} ",i,j ,input[i][j])
        }
        println!();
    }
    //assert_eq!(is_visible(&input, 3, 3), false);
    assert_eq!(is_score_single(&input, 1, 2), 4);
    assert_eq!(is_score_single(&input, 3, 2), 8);
}