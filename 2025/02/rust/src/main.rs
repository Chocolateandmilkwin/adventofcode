pub trait Wrap {
    fn wrap(self, min: i16, max: i16) -> i16;
}

impl Wrap for i16 {
    fn wrap(self, min: i16, max: i16) -> i16 {
        if self < min {
            max - (min - self - 1) % (max - min + 1)
        } else if self > max {
            min + (self - max - 1) % (max - min + 1)
        } else {
            self
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
            *a = (*a + p).wrap(0, 99);
            Some(*a)
        })
        .fold(0, |a, b| if b == 0 { a + 1 } else { a });
    println!("Hello, world! {:?} ", lines);
}
