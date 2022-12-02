use std::fs;

fn main() {
    // first();
    // first_b();
    day_2_a();
    day_2_b();
}

enum Shape {
    Rock,
    Paper,
    Scissors,

}

// 1 A X - Rock
// 2 B Y - Paper
// 3 C Z - Scissors
// 0 Lose
// 3 Draw
// 6 Win
fn day_2_a() {
    let contents = fs::read_to_string("/home/alex/development/advent2022/src/2.input").unwrap();
    fn decode_line(line: &str) -> (Shape, Shape) {
        let vec: Vec<&str> = line.split_whitespace().collect();

        let op = match vec[0] {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            _ => panic!()
        };
        let me = match vec[1] {
            "X" => Shape::Rock,
            "Y" => Shape::Paper,
            "Z" => Shape::Scissors,
            _ => panic!()
        };
        (op, me)
    }
    ;
    fn get_score(shapes: (Shape, Shape)) -> i32 {
        let res_score = match shapes {
            (Shape::Rock, Shape::Scissors) => 0,
            (Shape::Rock, Shape::Paper) => 6,
            (Shape::Scissors, Shape::Paper) => 0,
            (Shape::Scissors, Shape::Rock) => 6,
            (Shape::Paper, Shape::Rock) => 0,
            (Shape::Paper, Shape::Scissors) => 6,
            (_, _) => 3
        };
        let (_, me) = shapes;
        match me {
            Shape::Rock => res_score + 1,
            Shape::Paper => res_score + 2,
            Shape::Scissors => res_score + 3,
        }
    }

    let sum: i32 = contents.lines().map(decode_line).map(get_score).sum();
    println!("The sum is {sum}")
}

// X - Lose
// Y - Draw
// Z - Win
enum Result { Win, Draw, Lose }

fn day_2_b() {
    let contents = fs::read_to_string("/home/alex/development/advent2022/src/2.input").unwrap();
    fn decode_line(line: &str) -> (Shape, Result) {
        let vec: Vec<&str> = line.split_whitespace().collect();

        let op = match vec[0] {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            _ => panic!()
        };
        let result = match vec[1] {
            "X" => Result::Lose,
            "Y" => Result::Draw,
            "Z" => Result::Win,
            _ => panic!()
        };
        (op, result)
    }
    ;
    fn get_score(input: (Shape, Result)) -> i32 {
        let (shape, result) = input;
        match result {
            Result::Win => {
                match shape {
                    Shape::Rock => 6 + 2, // paper
                    Shape::Paper => 6 + 3, // scissors
                    Shape::Scissors => 6 + 1, // rock
                }
            }
            Result::Draw => {
                match shape {
                    Shape::Rock => 3 + 1, // rock
                    Shape::Paper => 3 + 2, // paper
                    Shape::Scissors => 3 + 3, // Scissors
                }
            }
            Result::Lose => {
                match shape {
                    Shape::Rock => 0 + 3, // Scissors
                    Shape::Paper => 0 + 1, // Rock
                    Shape::Scissors => 0 + 2, // paper
                }
            }
        }
    }
    let sum: i32 = contents.lines().map(decode_line).map(get_score).sum();
    println!("The sum is {sum}")
}


fn first() {
    let contents = fs::read_to_string("/home/phantom-il-alex/advent2022/src/1.input").unwrap();
    let contents = contents.split("\n\n");
    let max = contents.into_iter().fold(0, |max, current| {
        let sum: i32 = current.split("\n").into_iter().fold(0, |acc, x| acc + x.parse::<i32>().unwrap());
        if sum > max {
            sum
        } else {
            max
        }
    });
    println!("The max is {max}");
}

fn first_b() {
    let contents = fs::read_to_string("/home/phantom-il-alex/advent2022/src/1.input").unwrap();
    let contents = contents.split("\n\n");

    let mut sums: Vec<i32> = contents.into_iter().map(|current| {
        current.lines().into_iter().fold(0, |acc, x| acc + x.parse::<i32>().unwrap())
    }).collect();
    sums.sort();
    let sum = sums.as_slice()[sums.len() - 3..].to_vec().iter().fold(0, |a, x| a + x);
    println!("The max is {:?}", sum);
}