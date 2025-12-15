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

    fn follow_path(
        lines: &Vec<Vec<usize>>,
        cache: &mut Vec<Option<usize>>,
        start: usize,
        end: usize,
    ) -> usize {
        if start == end {
            1
        } else if let Some(cached) = cache[start] {
            cached
        } else {
            let count = lines[start]
                .iter()
                .fold(0usize, |a, &n| a + follow_path(lines, cache, n, end));
            cache[start] = Some(count);
            count
        }
    }

    let mut cache1 = vec![None; 16777216];
    let paths = follow_path(&lines, &mut cache1, YOU, OUT);
    println!("{:?}", paths);

    let mut cache_svr_fft = vec![None; 16777216];
    let paths2 = follow_path(&lines, &mut cache_svr_fft, SVR, FFT);
    println!("SVR -> FFT {:?}", paths2);

    let mut cache_fft_dac = vec![None; 16777216];
    let paths4 = follow_path(&lines, &mut cache_fft_dac, FFT, DAC);
    println!("FFT -> DAC {:?}", paths4);

    let mut cache_dac_out = vec![None; 16777216];
    let paths7 = follow_path(&lines, &mut cache_dac_out, DAC, OUT);
    println!("DAC -> OUT {:?}", paths7);
    println!("SVR -> OUT {:?}", paths2 * paths4 * paths7);

    let mut cache_svr_dac = vec![None; 16777216];
    let paths3 = follow_path(&lines, &mut cache_svr_dac, SVR, DAC);
    println!("SVR -> DAC {:?}", paths3);

    let mut cache_dac_fft = vec![None; 16777216];
    let paths5 = follow_path(&lines, &mut cache_dac_fft, DAC, FFT);
    println!("DAC -> FFT {:?}", paths5);

    let mut cache_fft_out = vec![None; 16777216];
    let paths6 = follow_path(&lines, &mut cache_fft_out, FFT, OUT);
    println!("FFT -> OUT {:?}", paths6);
    println!("SVR -> OUT {:?}", paths3 * paths5 * paths6);
}
