use std::fs;

fn main() {
    first();
    first_b();
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

    let mut sums:Vec<i32> = contents.into_iter().map(|current| {
        current.lines().into_iter().fold(0, |acc, x| acc + x.parse::<i32>().unwrap())

    }).collect();
    sums.sort();
    let sum = sums.as_slice()[sums.len()-3..].to_vec().iter().fold(0, |a, x| a + x);
    println!("The max is {:?}", sum);
}