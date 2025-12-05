use std::ops::RangeInclusive;

fn main() {
    let path = "./input.txt";
    let file = std::fs::read(path).unwrap();

    let string = String::from_utf8(file).unwrap();
    let mut split = string.split("\n\n");
    let mut ranges = split
        .next()
        .unwrap()
        .split("\n")
        .map(|l| l.split('-').collect::<Vec<&str>>())
        .map(|v| v[0].parse::<i64>().unwrap()..=v[1].parse::<i64>().unwrap())
        .collect::<Vec<RangeInclusive<i64>>>();
    ranges.sort_by(|a, b| a.end().cmp(b.end()));
    ranges.sort_by(|a, b| a.start().cmp(b.start()));

    let mut merged_ranges = Vec::<RangeInclusive<i64>>::with_capacity(100);
    let mut current_range = ranges[0].clone();
    for ran in ranges.iter().skip(1) {
        if ran.start() <= current_range.end() {
            if ran.end() > current_range.end() {
                current_range = current_range.start().clone()..=ran.end().clone();
            }
        } else {
            merged_ranges.push(current_range.clone());
            current_range = ran.clone();
        }
    }
    merged_ranges.push(current_range.clone());

    for ran in ranges.iter() {
        if !ranges
            .iter()
            .any(|r| r.contains(&ran.start()) && r.contains(&ran.end()))
        {
            println!("{:?}", ran);
        }
    }
    let ids = split
        .next()
        .unwrap()
        .split("\n")
        .map(|l| i64::from_str_radix(l, 10).unwrap())
        .collect::<Vec<i64>>();

    let count = ids.iter().fold(0, |a, id| {
        if merged_ranges.iter().any(|r| r.contains(&id)) {
            a + 1
        } else {
            a
        }
    });

    println!("{:?}", count);

    let fresh = merged_ranges
        .iter()
        .fold(0, |a, r| a + (r.end() - r.start() + 1));

    println!("{:?}", fresh);
}
