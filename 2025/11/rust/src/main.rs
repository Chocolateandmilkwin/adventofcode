fn main() {
    let path = "./input.txt";
    let file = std::fs::read(path).unwrap();

    const YOU: usize = {
        let b = b"you";
        (b[0] as usize) | ((b[1] as usize) << 8) | ((b[2] as usize) << 16)
    };
    const OUT: usize = {
        let b = b"out";
        (b[0] as usize) | ((b[1] as usize) << 8) | ((b[2] as usize) << 16)
    };
    const SVR: usize = {
        let b = b"svr";
        (b[0] as usize) | ((b[1] as usize) << 8) | ((b[2] as usize) << 16)
    };
    const DAC: usize = {
        let b = b"dac";
        (b[0] as usize) | ((b[1] as usize) << 8) | ((b[2] as usize) << 16)
    };
    const FFT: usize = {
        let b = b"fft";
        (b[0] as usize) | ((b[1] as usize) << 8) | ((b[2] as usize) << 16)
    };

    let mut lines = Vec::<Vec<usize>>::with_capacity(16777216);
    lines.resize(16777216, Vec::new());

    file.split(|b| *b == b'\n').for_each(|l| {
        let mut line = l
            .split(|b| *b == b' ')
            .map(|b| (b[0] as usize) | ((b[1] as usize) << 8) | ((b[2] as usize) << 16));
        let main = line.next().unwrap();
        lines[main] = line.collect::<Vec<usize>>();
    });

    fn follow_path(lines: &Vec<Vec<usize>>, start: usize, end: usize) -> usize {
        if start == end {
            1
        } else {
            lines[start]
                .iter()
                .fold(0usize, |a, &n| a + follow_path(lines, n, end))
        }
    }

    let paths = follow_path(&lines, YOU, OUT);

    println!("{:?}", paths);

    fn follow_path_2(lines: &Vec<Vec<usize>>, start: usize, end: usize, targets: u8) -> usize {
        if start == end {
            println!("Reached OUT with targets: {}", targets);
            if targets == 3 { 1 } else { 0 }
        } else {
            lines[start].iter().fold(0usize, |a, &n| {
                let mut t = targets.clone();
                if start == DAC {
                    t |= 1;
                }
                if start == FFT {
                    t |= 2;
                }
                a + follow_path_2(lines, n, end, t)
            })
        }
    }

    let paths2 = follow_path(&lines, SVR, FFT);

    println!("{:?}", paths2);
}
