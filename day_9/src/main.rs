use std::collections::HashSet;
fn main() {
    println!("Hello, world!");
    let input = include_str!("../9.input");
    let moves = input.lines().map(|line| {
        let mut parts = line.split_whitespace();
        let (letter, count) = (parts.next().unwrap(), parts.next().unwrap().parse::<u32>().unwrap());
        match letter {
            "U" => (count, 0, 1),
            "D" => (count, 0, -1),
            "L" => (count, -1, 0),
            "R" => (count, 1, 0),
            _ => todo!()
        }
    }).collect::<Vec<(u32, i32, i32)>>();
    let mut r = Rope::new();

    for (c, x, y) in moves.clone() {
        for _ in 0..c {
            r.move_head((x, y));
        }
    }
    println!("ans: {}", r.tail_history.len());

    let mut r2 =  Rope2::new();
    for (c, x, y) in moves.clone() {
        for _ in 0..c {
            r2.move_head((x, y));
        }
    }
    println!("ans: {}", r2.tail_history.len());
}


struct Rope2 {
    head: (i32, i32),
    tail: Vec<(i32,i32)>,
    pub tail_history: HashSet<(i32, i32)>,

}

impl Rope2 {
    pub fn new( ) -> Rope2 {
        Rope2 {
            head: (0, 0),
            tail: vec![(0,0); 9],
            tail_history: Default::default(),

        }
    }
    fn update_tail(&mut self, head: (i32, i32), index: usize) {
        let (head_x, head_y) = head;
        let (tail_x, tail_y) = self.tail[index];

        //non diag
        if (head_x == tail_x) && head_y.abs_diff(tail_y) == 2 ||
            (head_y == tail_y) && head_x.abs_diff(tail_x) == 2 {
            self.tail[index] = self.add(self.tail[index], ((head_x - tail_x) / 2, (head_y - tail_y) / 2));
        } else { // diag
            if head_x.abs_diff(tail_x) == 1 && head_y.abs_diff(tail_y) == 2 {
                let x = head_x - tail_x;
                let y = (head_y - tail_y) / 2;
                self.tail[index] = self.add(self.tail[index], (x, y));
            }
            if head_y.abs_diff(tail_y) == 1 && head_x.abs_diff(tail_x) == 2 {
                let x = (head_x - tail_x) / 2;
                let y = head_y - tail_y;
                self.tail[index] = self.add(self.tail[index], (x, y))
            }
            if head_y.abs_diff(tail_y) == 2 && head_x.abs_diff(tail_x) == 2 {
                let x = (head_x - tail_x) / 2;
                let y = (head_y - tail_y) / 2;
                self.tail[index] = self.add(self.tail[index], (x, y))
            }
        }


        self.tail_history.insert(*self.tail.last().unwrap());
    }
    pub fn move_head(&mut self, dir: (i32, i32)) {
        self.head = self.add(self.head, dir);
        self.update_tail(self.head, 0);

        for i in 0..self.tail.len() - 1{
            self.update_tail(self.tail[i], i + 1)
        }


    }

    fn add(&self, a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
        (a.0 + b.0, a.1 + b.1)
    }
}

struct Rope {
    head: (i32, i32),
    tail: (i32, i32),
    pub tail_history: HashSet<(i32, i32)>,
}

impl Rope {
    pub fn new() -> Rope {
        Rope {
            head: (0, 0),
            tail: (0, 0),
            tail_history: Default::default(),
        }
    }
    fn update_tail(&mut self) {
        let (head_x, head_y) = self.head;
        let (tail_x, tail_y) = self.tail;

        //non diag
        if (head_x == tail_x) && head_y.abs_diff(tail_y) == 2 ||
            (head_y == tail_y) && head_x.abs_diff(tail_x) == 2 {
            self.tail = self.add(self.tail, ((head_x - tail_x) / 2, (head_y - tail_y) / 2));
        } else { // diag
            if head_x.abs_diff(tail_x) == 1 && head_y.abs_diff(tail_y) == 2 {
                let x = head_x - tail_x;
                let y = (head_y - tail_y) / 2;
                self.tail = self.add(self.tail, (x, y));
            }
            if head_y.abs_diff(tail_y) == 1 && head_x.abs_diff(tail_x) == 2 {
                let x = (head_x - tail_x) / 2;
                let y = head_y - tail_y;
                self.tail = self.add(self.tail, (x, y))
            }
        }
        // println!("head_x: {}  head_y: {}, tail_x: {}, tail_y: {}", self.head.0, self.head.1, self.tail.0, self.tail.1);
        self.tail_history.insert(self.tail);
    }
    pub fn move_head(&mut self, dir: (i32, i32)) {
        self.head = self.add(self.head, dir);


        self.update_tail();
    }
    fn add(&self, a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
        (a.0 + b.0, a.1 + b.1)
    }
}

#[test]
fn test() {
    println!("test")
}