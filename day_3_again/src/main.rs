fn main() {
    let input = include_str!("../3.input");

    let sum: u32 = input.lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(p1, p2)| p1.chars()
            .find(|&c| p2.contains(c))
            .map(|c| {
                return if c.is_lowercase() {
                    c as u32 - 96
                } else {
                    c as u32 - 38
                };
            }).unwrap())
        .sum();


    println!("{:?}", sum);
    let sum: u32 = input.lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunk| chunk[0].chars().
            find(|&c| chunk[1].contains(c) && chunk[2].contains(c)).unwrap())
        .map(|c| return if c.is_lowercase() {
            c as u32 - 96
        } else {
            c as u32 - 38
        }).sum();
    println!("{:?}", sum);
}
