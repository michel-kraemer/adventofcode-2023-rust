use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let input = input.trim().chars().collect::<Vec<_>>();

    let mut score = 0;
    let mut non_canceled = 0;
    let mut i = 0;
    let mut level = 0;
    while i < input.len() {
        let mut c = input[i];
        match c {
            '{' => level += 1,
            '}' => {
                score += level;
                level -= 1;
            }
            '<' => {
                while i < input.len() && c != '>' {
                    c = input[i];
                    if c == '!' {
                        i += 1;
                    } else {
                        non_canceled += 1;
                    }
                    i += 1;
                }
                i -= 1; // don't skip beyond '>'
                non_canceled -= 2; // don't count '<' and '>'
            }
            _ => {}
        }
        i += 1;
    }

    println!("{}", score);
    println!("{}", non_canceled);
}
