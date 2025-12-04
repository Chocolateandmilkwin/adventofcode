fn main() {
    let path = "./input.txt";
    let file = std::fs::read(path).unwrap();

    let rolls = file
        .split(|num| *num == b'\n')
        .map(|l| {
            l.iter()
                .map(|c| if *c == b'@' { Some(()) } else { None })
                .collect::<Vec<Option<()>>>()
        })
        .collect::<Vec<Vec<Option<()>>>>();

    fn get_roll(rolls: &Vec<Vec<Option<()>>>, x: usize, y: usize) -> u8 {
        if x == 0 || y == 0 {
            return 0;
        }
        if rolls
            .get(y - 1)
            .and_then(|row| row.get(x - 1))
            .and_then(|&cell| cell)
            .is_some()
        {
            1
        } else {
            0
        }
    }

    fn iter_rolls(
        rolls: &Vec<Vec<Option<()>>>,
    ) -> impl Iterator<Item = (isize, isize, &Option<()>)> {
        rolls.iter().enumerate().flat_map(move |(y, v)| {
            v.iter()
                .enumerate()
                .map(move |(x, v)| (x as isize, y as isize, v))
        })
    }

    fn iter_grid(
        x_min: isize,
        x_max: isize,
        y_min: isize,
        y_max: isize,
    ) -> impl Iterator<Item = (isize, isize)> {
        (y_min..=y_max).flat_map(move |y| (x_min..=x_max).map(move |x| (x, y)))
    }

    const ROLL_FACTOR: i8 = 4;
    fn check_neighbors(rolls: &Vec<Vec<Option<()>>>, x: isize, y: isize, v: &Option<()>) -> isize {
        if v.is_some()
            && iter_grid(x - 1, x + 1, y - 1, y + 1).fold(0, |ga, (gx, gy)| {
                if x != gx || y != gy {
                    ga + get_roll(rolls, gx as usize, gy as usize) as isize
                } else {
                    ga
                }
            }) < ROLL_FACTOR as isize
        {
            1
        } else {
            0
        }
    }

    fn count_rolls(rolls: &Vec<Vec<Option<()>>>) -> isize {
        iter_rolls(&rolls)
            .map(|(x, y, v)| check_neighbors(&rolls, x + 1, y + 1, v))
            .fold(0, |a, b| a + b)
    }

    let count = count_rolls(&rolls);

    println!("{:?}", count);

    let mut second_count = 0;
    let mut new_rolls = rolls;
    loop {
        let level_count = count_rolls(&new_rolls);
        second_count += level_count;
        if (level_count) == 0 {
            break;
        }
        new_rolls = new_rolls
            .iter()
            .enumerate()
            .map(|(y, ve)| {
                ve.iter()
                    .enumerate()
                    .map(|(x, v)| {
                        if check_neighbors(&new_rolls, x as isize + 1, y as isize + 1, v) == 1 {
                            None
                        } else {
                            *v
                        }
                    })
                    .collect::<Vec<Option<()>>>()
            })
            .collect::<Vec<Vec<Option<()>>>>();
    }

    println!("{:?}", second_count);
}
