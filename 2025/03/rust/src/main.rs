fn main() {
    let path = "./input.txt";
    let file = std::fs::read(path).unwrap();

    //      ____  _____  _    _ _______ ______   ______ ____  _____   _____ ______
    //     |  _ \|  __ \| |  | |__   __|  ____| |  ____/ __ \|  __ \ / ____|  ____|
    //     | |_) | |__) | |  | |  | |  | |__    | |__ | |  | | |__) | |    | |__
    //     |  _ <|  _  /| |  | |  | |  |  __|   |  __|| |  | |  _  /| |    |  __|
    //     | |_) | | \ \| |__| |  | |  | |____  | |   | |__| | | \ \| |____| |____
    //     |____/|_|  \_\\____/   |_|  |______| |_|    \____/|_|  \_\\_____|______|
    let lines = file.split(|num| *num == b'\n').collect::<Vec<&[u8]>>();
    let jolts = lines
        .iter()
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
        .fold(0, |a, b| a + b);
    println!("Hello, world! {:?} ", jolts);

    let ln = lines
        .iter()
        .map(|l| {
            let mut lc = l
                .iter()
                .map(|v| *v - 48)
                .enumerate()
                .collect::<Vec<(usize, u8)>>();
            lc.sort_by(|(_ai, a), (_bi, b)| a.cmp(b));
            lc
        })
        .collect::<Vec<Vec<(usize, u8)>>>();

    println!("Hello, world! {:?} ", ln.last().unwrap());

    fn count(lines: &Vec<Vec<(usize, u8)>>, amount: usize) -> usize {
        if amount == 0 {
            return 0;
        }
        lines.iter().fold(0, |a, l| {
            let mut batts = l[(l.len() - amount)..]
                .iter()
                .collect::<Vec<&(usize, u8)>>();
            batts.sort_by(|(ai, _a), (bi, _b)| ai.cmp(bi).reverse());
            let jolt = batts.iter().enumerate().fold(0, |aa, (bi, (_bii, b))| {
                aa + ((*b as u64) * 10u64.pow(bi as u32))
            }) as usize;
            a + jolt
        })
    }

    let count1 = count(&ln, 2);

    println!("Hello, world! {:?} ", count1);
}
