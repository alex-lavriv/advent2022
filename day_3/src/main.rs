use std::fs;
fn main() {
    let contents = fs::read_to_string("/home/alex/development/advent2022/day_3/src/3.input").unwrap();
    let sum:u32 = contents.lines().map(solve_line).sum();
    println!("{sum}");

    let sum:u32 = contents.lines().collect::<Vec<_>>().chunks(3).map(find_same_letter).sum();
    println!("{sum}");

}
fn solve_line(line :&str) -> u32{
    let length = line.len() / 2;
    let first_part = &line[..length];
    let second_part = &line[length..];

    for current_char in first_part.chars(){
        if second_part.contains(current_char){
            return if current_char.is_lowercase() {
                current_char as u32 - 96
            } else {
                current_char as u32 - 38
            }
        }
    }
    return 0;

}
fn find_same_letter(split: &[&str]) -> u32{
    let (line1, line2, line3) = (split[0], split[1], split[2]);
    for current_char in line1.chars(){
        if line2.contains(current_char) && line3.contains(current_char){
            return if current_char.is_lowercase() {
                current_char as u32 - 96
            } else {
                current_char as u32 - 38
            }
        }
    }
    return 0;

}
#[test]
fn solve_line_test(){
    println!("test");
    println!("The answer is {}", solve_line("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"));

}

#[test]
fn solve_find_same_letter_test(){
    println!("test");
    println!("The answer is {}", find_same_letter(["vJrwpWtwJgWrhcsFMMfFFhFp","jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL","PmmdzqPrVvPwwTWBwg"]));

}