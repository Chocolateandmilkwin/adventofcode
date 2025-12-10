use std::{fmt::Debug, ops::RangeInclusive};

#[derive(PartialEq, Copy, Clone, Debug)]
struct Point2D {
    x: i64,
    y: i64,
}

impl Point2D {
    fn from_iter<I: Iterator<Item = i64>>(mut iter: I) -> Self {
        Self {
            x: iter.next().unwrap(),
            y: iter.next().unwrap(),
        }
    }
    fn area(&self, other: &Point2D) -> i64 {
        ((self.x - other.x).abs() + 1) * ((self.y - other.y).abs() + 1)
    }
    fn outline(&self, other: &Point2D) -> impl Iterator<Item = Point2D> {
        let xmin = self.x.min(other.x);
        let xmax = self.x.max(other.x);
        let ymin = self.y.min(other.y);
        let ymax = self.y.max(other.y);
        (xmin..xmax)
            .map(move |x| Point2D { x, y: ymin })
            .chain((ymin..ymax).map(move |y| Point2D { x: xmax, y }))
            .chain(
                ((xmin + 1)..=xmax)
                    .rev()
                    .map(move |x| Point2D { x, y: ymax }),
            )
            .chain(
                ((ymin + 1)..=ymax)
                    .rev()
                    .map(move |y| Point2D { x: xmin, y }),
            )
    }
}

fn main() {
    let path = "./input.txt";
    let file = std::fs::read(path).unwrap();

    let string = String::from_utf8(file).unwrap();
    let points = string
        .split("\n")
        .map(|l| Point2D::from_iter(l.split(",").map(|c| c.trim().parse::<i64>().unwrap())))
        .collect::<Vec<Point2D>>();

    let mut areas = points
        .iter()
        .enumerate()
        .flat_map(|(pi, pp)| {
            points
                .iter()
                .enumerate()
                .filter(move |(qi, _qp)| pi > *qi)
                .map(move |(qi, qp)| (pp.area(qp), pi, qi))
                .filter(|d| d.0 != 0)
        })
        .collect::<Vec<(i64, usize, usize)>>();
    areas.sort_by(|a, b| a.0.cmp(&b.0));

    let last = areas.last().unwrap();
    println!("{:?}", last.0);

    let (mut ranges_x, mut ranges_y) = points
        .iter()
        .zip(points.iter().skip(1).chain(points.iter().take(1)))
        .fold(
            (
                Vec::<(i64, RangeInclusive<i64>)>::with_capacity(2000),
                Vec::<(i64, RangeInclusive<i64>)>::with_capacity(2000),
            ),
            |mut a, (p1, p2)| {
                if p1.x == p2.x {
                    a.1.push((p1.x, p1.y.min(p2.y)..=p1.y.max(p2.y)));
                } else {
                    a.0.push((p1.y, p1.x.min(p2.x)..=p1.x.max(p2.x)));
                }
                a
            },
        );
    ranges_x.sort_by(|a, b| a.0.cmp(&b.0));
    ranges_y.sort_by(|a, b| a.0.cmp(&b.0));

    let point_in_polygon = |point: Point2D| {
        if ranges_x
            .iter()
            .filter(|(y, _r)| y == &point.y)
            .any(|(_y, r)| r.contains(&point.x))
        {
            return true;
        } else if ranges_y
            .iter()
            .filter(|(x, _r)| x == &point.x)
            .any(|(_x, r)| r.contains(&point.y))
        {
            return true;
        } else if ranges_y
            .iter()
            .filter(|(x, _r)| x >= &point.x)
            .fold(
                0i64,
                |a, (_x, r)| {
                    if r.contains(&point.y) { a + 1 } else { a }
                },
            )
            % 2
            == 1
        {
            return true;
        } else {
            return false;
        }
    };

    let max = areas
        .iter()
        .rev()
        .filter(|(_a, p1, p2)| {
            println!("Checking {:?} {:?} area {:?}", points[*p1], points[*p2], _a);
            points[*p1]
                .outline(&points[*p2])
                .all(|pt| point_in_polygon(pt))
        })
        .take(1)
        .next()
        .unwrap();

    println!("{:?} ", max);
}
