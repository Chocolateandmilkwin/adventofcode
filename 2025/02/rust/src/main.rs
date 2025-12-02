use std::ops::RangeInclusive;

pub trait CountDigits {
    fn count_digits(self) -> u16;
}

impl CountDigits for u64 {
    fn count_digits(self) -> u16 {
        if self == 0 {
            return 1;
        }
        let n_f64 = self as f64;
        let log_value = n_f64.log10();
        log_value.floor() as u16 + 1
    }
}

fn main() {
    let path = "./input.txt";
    let file = std::fs::read(path).unwrap();

    //      ____  _____  _    _ _______ ______   ______ ____  _____   _____ ______
    //     |  _ \|  __ \| |  | |__   __|  ____| |  ____/ __ \|  __ \ / ____|  ____|
    //     | |_) | |__) | |  | |  | |  | |__    | |__ | |  | | |__) | |    | |__
    //     |  _ <|  _  /| |  | |  | |  |  __|   |  __|| |  | |  _  /| |    |  __|
    //     | |_) | | \ \| |__| |  | |  | |____  | |   | |__| | | \ \| |____| |____
    //     |____/|_|  \_\\____/   |_|  |______| |_|    \____/|_|  \_\\_____|______|
    let lines = file
        .split(|num| *num == b',')
        .map(|sr| sr.split(|num| *num == b'-'))
        .map(|srt| {
            srt.map(|y| {
                String::from_utf8(y.to_vec())
                    .unwrap()
                    .parse::<u64>()
                    .unwrap()
            })
        })
        .map(|mut v| (v.next().unwrap(), v.next().unwrap()))
        .flat_map(|r| {
            (r.0..=r.1)
                .map(|i| (i, i.count_digits()))
                .filter(|i| i.1 % 2 == 0)
                .map(|i| (i.0, i.1, 10u64.pow((i.1 / 2).into())))
                .map(|i| (i.0, i.1, i.2, i.0 / i.2))
                .map(|i| (i.0, i.1, i.2, i.3, i.0 - i.3 * i.2 as u64))
                .filter(|i| i.3 == i.4)
                .map(|i| i.0)
        })
        .fold(0, |acc, i| acc + i);

    println!("Hello, world! {:?} ", lines);
}
