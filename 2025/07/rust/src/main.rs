use std::{fmt::Debug, io::Empty};

#[derive(PartialEq, Copy, Clone)]
enum Operation {
    Empty,
    Splitter,
    BSplitter,
    Source,
    Beam,
}
impl Debug for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Empty => write!(f, "."),
            Operation::Splitter => write!(f, "^"),
            Operation::BSplitter => write!(f, "*"),
            Operation::Source => write!(f, "S"),
            Operation::Beam => write!(f, "|"),
        }
    }
}

fn main() {
    let path = "./input.txt";
    let file = std::fs::read(path).unwrap();

    let string = String::from_utf8(file).unwrap();
    let lines = string
        .split("\n")
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => Operation::Empty,
                    '^' => Operation::Splitter,
                    'S' => Operation::Source,
                    '|' => Operation::Beam,
                    _ => panic!("Unknown character"),
                })
                .collect::<Vec<Operation>>()
        })
        .collect::<Vec<Vec<Operation>>>();

    // println!("{:?}", lines);

    let mut sum = 0;
    let fline = lines.iter().skip(1).fold(
        (
            lines.first().unwrap().clone(),
            lines
                .first()
                .unwrap()
                .clone()
                .iter()
                .map(|o| if *o != Operation::Empty { 1 } else { 0 })
                .collect::<Vec<usize>>(),
        ),
        |(mut a, mut t), l| {
            for o in &a {
                print!("{:?} ", o);
            }
            println!();

            for i in 0..l.len() {
                match a[i] {
                    Operation::Source | Operation::Beam => {
                        if l[i] == Operation::Splitter {
                            a[i] = Operation::BSplitter;
                        } else {
                            a[i] = Operation::Beam;
                        }
                    }
                    Operation::BSplitter => {
                        sum += 1;
                        a[i] = Operation::Empty;
                        if i > 0
                            && let Some(left) = a.get_mut(i - 1)
                        {
                            *left = Operation::Beam;
                        }
                        if let Some(right) = a.get_mut(i + 1) {
                            *right = Operation::Beam;
                        }
                        let val = t[i];
                        if i > 0
                            && let Some(left) = t.get_mut(i - 1)
                        {
                            *left += val;
                        }
                        if let Some(right) = t.get_mut(i + 1) {
                            *right += val;
                        }
                        t[i] = 0;
                    }
                    _ => {
                        a[i] = Operation::Empty;
                    }
                }
            }
            (a, t)
        },
    );

    for o in &fline.0 {
        print!("{:?} ", o);
    }
    println!();
    for o in &fline.1 {
        print!("{:?} ", o);
    }
    println!();

    println!("{:?}", sum);
    println!("{:?}", fline.1.iter().sum::<usize>());
}
