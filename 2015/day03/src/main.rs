use std::{collections::HashSet, fs};

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");

        let mut santa_x = 0i64;
        let mut santa_y = 0i64;
        let mut robo_x = 0i64;
        let mut robo_y = 0i64;
        let mut seen = HashSet::new();
        seen.insert((santa_x, santa_y));

        for (i, c) in input.chars().enumerate() {
            let (x, y) = if part1 || i % 2 == 0 {
                (&mut santa_x, &mut santa_y)
            } else {
                (&mut robo_x, &mut robo_y)
            };

            match c {
                'v' => *y += 1,
                '^' => *y -= 1,
                '>' => *x += 1,
                '<' => *x -= 1,
                _ => panic!(),
            }

            seen.insert((*x, *y));
        }

        println!("{}", seen.len());
    }
}
