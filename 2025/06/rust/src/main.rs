use std::fmt::Debug;

enum Operation {
    Add,
    Multiply,
}
impl Debug for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Add => write!(f, "+"),
            Operation::Multiply => write!(f, "*"),
        }
    }
}

fn main() {
    let path = "./input.txt";
    let file = std::fs::read(path).unwrap();

    let string = String::from_utf8(file).unwrap();
    let values = string.split("\n").collect::<Vec<&str>>();
    let (last, lines) = values.split_last().unwrap();

    let last2 = last
        .split(" ")
        .filter(|s| *s != "")
        .map(|c| match c {
            "+" => Operation::Add,
            _ => Operation::Multiply,
        })
        .collect::<Vec<Operation>>();

    let lines2 = lines
        .iter()
        .map(|l| l.split(" ").filter(|s| *s != "").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let lines_pivot = last2
        .iter()
        .enumerate()
        .map(|(i, v)| (v, lines2.iter().map(|l| l[i]).collect::<Vec<&str>>()))
        .collect::<Vec<(&Operation, Vec<&str>)>>();

    let lines_pivot_num = lines_pivot.iter().map(|(o, v)| {
        (
            o,
            v.iter()
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<i64>>(),
        )
    });

    let results = lines_pivot_num
        .map(|(o, vals)| match o {
            Operation::Add => vals.iter().sum::<i64>(),
            Operation::Multiply => vals.iter().product::<i64>(),
        })
        .collect::<Vec<i64>>();

    let sum = results.iter().sum::<i64>();
    println!("{:?}", sum);

    let lines_ceph = last
        .char_indices()
        .filter(|(i, c)| *c != ' ')
        .collect::<Vec<(usize, char)>>();

    let lines_ceph_pivot = lines_ceph
        .iter()
        .zip(
            lines_ceph
                .iter()
                .skip(1)
                .chain(std::iter::once(&(last.len(), ' '))),
        )
        .map(|((ai, ac), (bi, _bc))| {
            (
                match ac {
                    '+' => Operation::Add,
                    _ => Operation::Multiply,
                },
                (*ai..*bi)
                    .map(|i| {
                        lines
                            .iter()
                            .map(|s| s.chars().nth(i).unwrap())
                            .filter(|c| *c != ' ')
                            .map(|c| c.to_digit(10).unwrap())
                            .rev()
                            .enumerate()
                            .fold(0, |a, (i, v)| 10u32.pow(i as u32) * v + a)
                            as i64
                    })
                    .filter(|v| *v != 0)
                    .collect::<Vec<i64>>(),
            )
        })
        .collect::<Vec<(Operation, Vec<i64>)>>();

    println!("{:?}", lines_ceph_pivot);

    let results2 = lines_ceph_pivot
        .iter()
        .map(|(o, vals)| match o {
            Operation::Add => vals.iter().sum::<i64>(),
            Operation::Multiply => vals.iter().product::<i64>(),
        })
        .collect::<Vec<i64>>();

    let sum2 = results2.iter().sum::<i64>();
    println!("{:?}", sum2);
}
