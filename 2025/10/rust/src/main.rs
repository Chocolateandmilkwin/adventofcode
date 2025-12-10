fn main() {
    let path = "./input.txt";
    let file = std::fs::read(path).unwrap();

    let string = String::from_utf8(file).unwrap();
    let lines = string
        .split("\n")
        .map(|l| {
            let line = l.split(" ").collect::<Vec<&str>>();
            let buttons = line[0][1..line[0].len() - 1]
                .chars()
                .map(|c| c == '.')
                .collect::<Vec<bool>>();
            let wiring = line[1..line.len() - 1]
                .iter()
                .map(|w| {
                    w[1..w.len() - 1]
                        .split(",")
                        .map(|s| s.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>()
                })
                .collect::<Vec<Vec<i32>>>();
            let last = line[line.len() - 1];
            let jolt = last[1..last.len() - 1]
                .split(",")
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            (buttons, wiring, jolt)
        })
        .collect::<Vec<(Vec<bool>, Vec<Vec<i32>>, Vec<i32>)>>();

    //println!("{:?}", lines);

    fn apply_buttons(buttons: &mut Vec<bool>, wiring: &Vec<i32>) {
        for w in wiring {
            buttons[*w as usize] = !buttons[*w as usize];
        }
    }

    fn buttons_recusive(buttons: &Vec<bool>, wiring: &Vec<Vec<i32>>, mut depth: i32) -> bool {
        depth -= 1;
        if depth < 0 {
            return false;
        }
        wiring.iter().any(|w| {
            let mut b = buttons.clone();
            apply_buttons(&mut b, w);
            if b.iter().all(|b| *b) {
                true
            } else {
                buttons_recusive(&b, wiring, depth)
            }
        })
    }

    fn apply_joltage(joltage: &mut Vec<i32>, wiring: &Vec<i32>) {
        for w in wiring {
            joltage[*w as usize] += 1;
        }
    }

    fn joltage_recusive(
        joltage: &Vec<i32>,
        wiring: &Vec<Vec<i32>>,
        mut depth: i32,
        target: &Vec<i32>,
    ) -> bool {
        depth -= 1;
        if depth < 0 {
            return false;
        }
        wiring.iter().any(|w| {
            let mut b = joltage.clone();
            apply_joltage(&mut b, w);
            if b.iter().zip(target.iter()).all(|(j, t)| *j == *t) {
                true
            } else if b.iter().zip(target.iter()).any(|(j, t)| *j > *t) {
                false
            } else {
                joltage_recusive(&b, wiring, depth, target)
            }
        })
    }

    let steps = lines.iter().enumerate().fold(
        (0i64, 0i64),
        |(but_step, jolt_step), (c, (b, w, target_jolt))| {
            let bs = (1..).find_map(|i| {
                if buttons_recusive(b, w, i) {
                    Some(i)
                } else {
                    None
                }
            });
            let jolts = b.iter().map(|_a| 0i32).collect::<Vec<i32>>();
            let js = (1..).find_map(|i| {
                println!("Checked depth {:?}", i);
                if joltage_recusive(&jolts, w, i, target_jolt) {
                    Some(i)
                } else {
                    None
                }
            });
            println!("Line {}: Steps {:?}", c + 1, bs);
            (
                but_step + bs.unwrap_or(0) as i64,
                jolt_step + js.unwrap_or(0) as i64,
            )
        },
    );

    println!("Button steps: {}", steps.0);
    println!("Jolt steps: {}", steps.1);
}
