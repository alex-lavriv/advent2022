use std::fs;
fn main() {
    first()
}


fn first(){
    let contents = fs::read_to_string("/home/phantom-il-alex/development/advent2022/src/1.input").unwrap();
    let contents = contents.split("\n\n");
    let max = contents.into_iter().fold(0, |max, current|{
        let sum:i32 = current.split("\n").into_iter().fold(0, |acc, x| acc + x.parse::<i32>().unwrap());
        if sum > max {
            sum
        }
        else {
            max
        }
    });
    println!("The max is {max}");
}