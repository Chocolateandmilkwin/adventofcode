use std::fmt::Debug;

#[derive(PartialEq, Copy, Clone, Debug)]
struct Point3D {
    x: i64,
    y: i64,
    z: i64,
    circuit: usize,
}

impl Point3D {
    fn from_iter<I: Iterator<Item = i64>>(c: usize, mut iter: I) -> Self {
        Self {
            x: iter.next().unwrap(),
            y: iter.next().unwrap(),
            z: iter.next().unwrap(),
            circuit: c,
        }
    }
    fn distance_to(&self, other: &Point3D) -> i64 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }
}

fn main() {
    let path = "./input.txt";
    let file = std::fs::read(path).unwrap();

    let string = String::from_utf8(file).unwrap();
    let mut points = string
        .split("\n")
        .enumerate()
        .map(|(i, l)| Point3D::from_iter(i, l.split(",").map(|c| c.trim().parse::<i64>().unwrap())))
        .collect::<Vec<Point3D>>();

    let mut distances = points
        .iter()
        .enumerate()
        .flat_map(|(pi, pp)| {
            points
                .iter()
                .enumerate()
                .filter(move |(qi, _qp)| pi > *qi)
                .map(move |(qi, qp)| (pp.distance_to(qp), pi, qi))
                .filter(|d| d.0 != 0)
        })
        .collect::<Vec<(i64, usize, usize)>>();
    distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

    // println!("{:?}", distances);
    // println!("{:?}", distances.len());

    let mut circuits = (0..points.len())
        .map(|i| vec![i])
        .collect::<Vec<Vec<usize>>>();

    // for (_dist, p1, p2) in distances.iter().take(1000) {
    //     let c1 = points[*p1].circuit;
    //     let c2 = points[*p2].circuit;
    //     if circuits[c1].contains(p2) {
    //         print!("Skipping same circuit\n");
    //         continue;
    //     }
    //     connections += 1;
    //     let (first_part, second_part) = circuits.split_at_mut(1);
    //     circuits[c1].extend(circuits[c2].iter());

    //     points[*p2].circuit = points[*p1].circuit;
    // }

    // circuits.sort();

    let mut connections = 0;
    let test = distances.iter().take(1000).fold(
        (0..points.len())
            .map(|i| vec![i])
            .collect::<Vec<Vec<usize>>>(),
        |mut a, (_dist, p1i, p2i)| {
            let c1 = points[*p1i].circuit;
            let c2 = points[*p2i].circuit;
            if a[c1].contains(p2i) || a[c2].contains(p1i) {
                return a;
            }
            connections += 1;
            let mut c2_points = a[c2].clone();
            a[c1].append(&mut c2_points);
            a[c2].clear();
            println!("{:?}", a);
            a
        },
    );
    let mut tests = test
        .iter()
        .map(|v| v.len())
        .filter(|l| *l > 0)
        .collect::<Vec<usize>>();
    tests.sort();

    // println!("{:?}", circuits);
    println!("{:?}", connections);
    println!("{:?}", tests);
    println!("{:?}", tests.iter().sum::<usize>());
    println!("{:?}", tests.iter().rev().take(3).product::<usize>());
    // println!("{:?}", circuits.iter().sum::<u64>());
    // println!("{:?}", circuits.iter().rev().take(3).product::<u64>());
}
