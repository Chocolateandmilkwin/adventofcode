pub trait Wrap {
    fn wrap(self) -> (i16, i16);
}

impl Wrap for i16 {
    fn wrap(self) -> (i16, i16) {
        if self < 0 {
            if self < -100 {
                return (self % 100, ((self / 100) - 1).abs());
            } else {
                return (self + 100, 1);
            }
        } else if self >= 100 {
            if self >= 200 {
                return (self % 100, self / 100 - 1);
            } else {
                return (self - 100, 1);
            }
        } else {
            return (self, 0);
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
            let found = (*a + p).wrap();
            *a = found.0;
            Some(found)
        })
        .fold((0, 0), |(a1, a2), (b1, b2)| {
            (if b1 == 0 { a1 + 1 } else { a1 }, a2 + b2)
        });
    println!("Hello, world! {:?} ", lines);

    // println!("Hello, world! {:?} ", (50i16 - 68i16).wrap());
    // println!("Hello, world! {:?} ", (82i16 - 30i16).wrap());
    // println!("Hello, world! {:?} ", (52i16 + 48i16).wrap());
    // println!("Hello, world! {:?} ", (50i16 + 250i16).wrap());
    // println!("Hello, world! {:?} ", (50i16 - 100i16).wrap());
}
