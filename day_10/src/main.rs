fn main() {
    println!("Hello, world!");
    let input = include_str!("../10.input");
    let input = parse_input(input);
    solve(&input);
    solve2(&input);
}

enum Command {
    ADDX(i32),
    NOOP,
}

fn solve2(commands: &Vec<Command>) {
    let mut cycle_count = 0;
    let mut register_x = 1;

    for command in commands {
        match command {
            Command::NOOP => {
                draw(cycle_count, register_x);
                cycle_count += 1;
            }
            Command::ADDX(v) => {
                for i in 0..2 {
                    draw(cycle_count, register_x);
                    if i == 1 {
                        register_x += v;
                    }
                    cycle_count += 1;
                }
            }
        }
    }
}

fn draw(cycle: i32, pos_register: i32) {
    if cycle % 40 == 0 {
        println!();
    }
    let current_row_pos = cycle % 40;
    if current_row_pos == pos_register || current_row_pos == pos_register + 1 || current_row_pos == pos_register - 1 {
        print!("#");
    } else {
        print!(".");
    }
}


fn solve(commands: &Vec<Command>) {
    let mut cycle_count = 0;
    let mut register_x = 1;
    let mut signal = 0;
    let check_cycle = [20, 60, 100, 140, 180, 220];
    for command in commands {
        cycle_count += 1;
        if check_cycle.contains(&cycle_count) {
            signal += cycle_count * register_x;
            //  println!("sig: {signal} cy: {cycle_count} regi {register_x} cycle*register {}", cycle_count * register_x);
        }

        match command {
            Command::NOOP => {}
            Command::ADDX(v) => {
                for i in 0..2 {
                    // println!("sig: {signal} cy: {cycle_count} regi {register_x} cycle*register {}", cycle_count * register_x);

                    if i == 1 {
                        cycle_count += 1;
                        if check_cycle.contains(&cycle_count) {
                            signal += cycle_count * register_x;
                            //  println!("sig: {signal} cy: {cycle_count} regi {register_x} cycle*register {}", cycle_count * register_x);
                        }
                        register_x += v;
                    }
                }
            }
        }
    }
    println!("Signal: {signal}");
}

fn parse_input(input: &str) -> Vec<Command> {
    input.lines().map(|line: &str| {
        if line.starts_with("addx") {
            let value = line.split_whitespace().into_iter().last().unwrap().parse::<i32>().unwrap();
            Command::ADDX(value)
        } else if line.starts_with("noop") {
            Command::NOOP
        } else {
            panic!()
        }
    }).collect()
}

#[test]
fn test() {
    println!("My test");
    let input = include_str!("../10.example");
    parse_input(input);
}