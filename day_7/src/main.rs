use std::cell::RefCell;
use std::rc::Rc;

enum NodeType {
    Folder((String, usize)),
    File((String, usize)),
}

fn main() {
    let input = include_str!("../7.input");
    let input = text_to_tree(input);
    println!("{}", input.borrow().print());

    let (acc, sum) = calc_sum(input, 0);
    println!("Total acc: {acc} sum {sum}");
    let input = include_str!("../7.input");
    let input = text_to_tree(input);
    let mut min = 70000000;
    let (min, _) = find_min(input, min);
    println!("Total min: {min}");
}

fn find_min(node: Rc<RefCell<TreeNode>>, mut min: usize) -> (usize, usize) {
    const TOTAL: usize = 70000000;
    const REQUIRED: usize = 30000000;
    const USED: usize = 43441553;

    const UNUSED: usize = TOTAL - USED;
    const NEEDED: usize = REQUIRED - UNUSED;
    let mut g_sum = 0;
    match &node.borrow().value {
        NodeType::File((_name, size)) => {
            return (min, *size);
        }
        NodeType::Folder((_name, _size)) => {
            for child in &node.borrow().children {
                let (inner_min, sum) = find_min(Rc::clone(&child), min);
                g_sum += sum;
                min = inner_min;
            }
        }
    }

    if (UNUSED + g_sum) > REQUIRED && g_sum < min {
        return (g_sum, g_sum);
    }
    (min, g_sum)
}

fn calc_sum(node: Rc<RefCell<TreeNode>>, mut acc: usize) -> (usize, usize) {
    const MAX_SIZE: usize = 100000;
    let mut g_sum = 0;
    match &node.borrow().value {
        NodeType::File((_name, size)) => {
            //  println!("file: {_name}");
            (0, *size)
        }
        NodeType::Folder((_name, _size)) => {
            println!("folder: {_name}");
            for child in &node.borrow().children {
                let (new_acc, sum) = calc_sum(Rc::clone(&child), 0);
                acc += new_acc;
                g_sum += sum;
            }
            println!("folder {_name} total size: {g_sum}");
            if g_sum < MAX_SIZE {
                acc += g_sum;
            }
            (acc, g_sum)
        }
    }
}

fn text_to_tree(text: &str) -> Rc<RefCell<TreeNode>> {
    let root = Rc::new(RefCell::new(TreeNode { value: NodeType::Folder(("/".to_string(), 0)), children: vec![], parent: None }));

    let mut current = Rc::clone(&root);


    for line in text.lines().skip(1) {
        if line.starts_with("$ cd") {
            let last = line.split_whitespace().last().unwrap();
            if last == ".." { // cd ..
                let current_clone = Rc::clone(&current);
                current = Rc::clone(current_clone.borrow().parent.as_ref().unwrap());
            } else { // cd dir
                let folder_name = last;
                let current_folder = Rc::clone(&current);
                for current_node in &current_folder.borrow().children {
                    if let NodeType::Folder((name, _)) = &current_node.borrow().value {
                        if name == folder_name {
                            current = Rc::clone(&current_node);
                        }
                    }
                }
            }
        } else if line.starts_with("$ ls") {} else { // sizes
            let mut parts = line.split_whitespace();

            match (parts.next(), parts.next()) {
                (Some(first), Some(second)) => {
                    if first == "dir" {
                        let node = Rc::new(RefCell::new(TreeNode {
                            value: NodeType::Folder((second.to_string(), 0)),
                            children: vec![],
                            parent: None,
                        }));
                        node.borrow_mut().parent = Some(Rc::clone(&current));
                        current.borrow_mut().children.push(Rc::clone(&node));
                    } else {
                        let size = first.parse::<usize>().unwrap();
                        let node = Rc::new(RefCell::new(TreeNode {
                            value: NodeType::File((second.to_string(), size)),
                            children: vec![],
                            parent: None,
                        }));
                        node.borrow_mut().parent = Some(Rc::clone(&current));
                        current.borrow_mut().children.push(Rc::clone(&node));
                    }
                }
                (_, _) => panic!()
            };
        }
    }

    root
}

#[test]
fn test() {
    println!("Hello, world!");
    let input = include_str!("../7.test");
    let input = text_to_tree(input);
    println!("{}", input.borrow().print());
}


// #[derive(PartialEq)]
struct TreeNode {
    pub value: NodeType,
    pub children: Vec<Rc<RefCell<TreeNode>>>,
    pub parent: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn add_child(&mut self, new_node: Rc<RefCell<TreeNode>>) {
        self.children.push(new_node);
    }

    pub fn print(&self) -> String {
        return match &self.value {
            NodeType::File((name, size)) => format!("{name} {size}"),
            NodeType::Folder((_name, _size)) => {
                String::from("[")
                    + &self
                    .children
                    .iter()
                    .map(|tn| tn.borrow().print())
                    .collect::<Vec<String>>()
                    .join(", ")
                    + "]"
            }
        };
    }
}

