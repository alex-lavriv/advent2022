use std::convert::TryInto;

fn main() {
    println!("Hello, world!");
    let input = include_str!("../12.input");
    let (mut board, (x,y )) = parse(input);
    let start = (x,y);
    println!("start {}, {}", x, y);
    board[x][y] = 255;
    let x = x.try_into().unwrap();
    let y = y.try_into().unwrap();
    print_board(&board);
    let min1 = solve(x + 1, y, &board, 1, 255, vec![start]);
    let min2 = solve(x, y + 1, &board, 1, 255, vec![start]);
    let min3 = solve(x - 1, y, &board, 1, 255, vec![start]);
    let min4 = solve(x, y - 1, &board, 1, 255, vec![start]);

    let min = *[min1, min2, min3, min4].iter().min().unwrap() + 1;
    println!("The res is {min}");
}
fn print_board(board : &Vec<Vec<u8>>){
    for line in board {
        for c in line {
            if *c == 255 {
                print!("{}", *c as char);
            }else{
                print!("_");
            }
        }
        println!();
    }
    println!();
}

fn solve(x: i32, y: i32,  board: &Vec<Vec<u8>>, count: usize, prev_value: u8, mut path: Vec<(usize, usize)>) -> usize {
  //  println!("x: {x}, y: {y}");
    if x < 0 || y < 0 || x == board.len().try_into().unwrap() || y == board[0].len().try_into().unwrap() {
        return  board.len() * board[0].len();
    }

    let x = x as usize;
    let y = y as usize;
    let max = board.len() * board[x].len();

    if path.contains(&(x,y)){
       // println!("BEEN HERE X: {x} Y: {y}");
        return max;
    }
    path.push((x,y));

    let current_val = board[x][y];



    if current_val == 122 {
        println!("found {count} path {:?}, length path.length: {}", path, path.len());
        return count;
    }
    if  current_val - 1 > prev_value{
        return max;
    }


  //  board[x][y] = 255;
   // print_board(&board);

    let x = x.try_into().unwrap();
    let y = y.try_into().unwrap();

  //  println!("NEXT STEP");
    let min1 = solve(x + 1, y, &board, count + 1, current_val, path.clone());
    let min2 = solve(x, y + 1, &board, count + 1, current_val, path.clone());
    let min3 = solve(x - 1, y, &board, count + 1, current_val, path.clone());
    let min4 = solve(x, y - 1, &board, count + 1, current_val, path.clone());

   let min = *[min1, min2, min3, min4].iter().min().unwrap();
   // println!("The min is {min}");
    min
}


fn parse(input: &str) -> (Vec<Vec<u8>>, (usize, usize)) {
    let mut result = Vec::new();
    let mut start = (0, 0);
    for (i, line) in input.lines().enumerate() {
        result.push(line.chars().enumerate().map(|(j, c)| {
            if c == 'S' {
                start = (i, j);
            }
            c as u8
        }).collect())
    }
    (result, start)
}