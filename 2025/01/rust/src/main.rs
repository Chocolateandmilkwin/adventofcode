pub trait Wrap {
    fn wrap(self, min: i16, max: i16) -> (i16, i16);
}

impl Wrap for i16 {
    fn wrap(self, min: i16, max: i16) -> (i16, i16) {
        let span = max - min + 1;
        if self < min {
            (max - (min - self - 1) % span, (self - min) / span)
        } else if self > max {
            (min + (self - max - 1) % span, (self - min) / span)
        } else {
            (self, 0)
        }
    }
}

fn main() {
    let path = "./input.txt";
    let file = std::fs::read(path).unwrap();
    let lines = file
        .split(|num| *num == b'\n')
        .map(|b| b.split_at(1))
        .map(|b| (b.0[0], String::from_utf8(b.1.to_vec()).unwrap()))
        .map(|s| {
            if s.0 == b'L' {
                -s.1.parse::<i16>().unwrap()
            } else {
                s.1.parse::<i16>().unwrap()
            }
        })
        .scan(50, |a, p| {
            let found = (*a + p).wrap(0, 99);
            *a = found.0;
            Some(found)
        })
        .fold((0, 0), |(a1, a2), b| {
            (
                if b.0 == 0 { a1 + 1 } else { a1 },
                if b.0 == 0 { a2 + 1 } else { a2 } + b.1.abs(),
            )
        });

    println!("Hello, world! {:?} ", (-47).wrap(0, 99));
    println!("Hello, world! {:?} ", lines);
}
