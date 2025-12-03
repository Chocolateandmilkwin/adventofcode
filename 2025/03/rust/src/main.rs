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
        .split(|num| *num == b'\n')
        .map(|l| {
            let m = l
                .iter()
                .rev()
                .map(|b| b - 48)
                .enumerate()
                .max_by(|a, b| a.1.cmp(&b.1))
                .unwrap();
            let i = l.len() - m.0;
            if m.0 == 0 {
                (l[..i - 1].iter().rev().map(|b| b - 48).max().unwrap() as i32) * 10 + (m.1 as i32)
            } else {
                (m.1 as i32) * 10 + (l[i + 1..].iter().map(|b| b - 48).max().unwrap() as i32)
            }
        })
        .map(|v| {
            println!("{:?}", v);
            v
        })
        .fold(0, |a, b| a + b);
    println!("Hello, world! {:?} ", lines);
}
