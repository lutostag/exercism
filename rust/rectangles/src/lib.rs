use std::collections::HashSet;
type Loc = (usize, usize);

fn down<'a>(loc: Loc, diagram: &'a [Vec<char>]) -> impl Iterator<Item = Loc> + 'a {
    (loc.0 + 1..diagram.len())
        .map(move |y| ((y, loc.1), &diagram[y][loc.1]))
        .take_while(|(_loc, &c)| c == '+' || c == '|')
        .filter_map(|(loc, c)| match c {
            '+' => Some(loc),
            _ => None,
        })
}

fn right<'a>(loc: Loc, diagram: &'a [Vec<char>]) -> impl Iterator<Item = Loc> + 'a {
    (loc.1 + 1..diagram[0].len())
        .map(move |x| ((loc.0, x), &diagram[loc.0][x]))
        .take_while(|(_loc, &c)| c == '+' || c == '-')
        .filter_map(|(loc, c)| match c {
            '+' => Some(loc),
            _ => None,
        })
}

pub fn count(lines: &[&str]) -> u32 {
    let diagram: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    let mut rectangles = 0;

    for (y, line) in diagram.iter().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            if c == '+' {
                let right_down: HashSet<_> = right((y, x), &diagram)
                    .flat_map(|loc| down(loc, &diagram))
                    .collect();
                let down_right: HashSet<_> = down((y, x), &diagram)
                    .flat_map(|loc| right(loc, &diagram))
                    .collect();
                rectangles += right_down.intersection(&down_right).count();
            }
        }
    }
    rectangles as u32
}
