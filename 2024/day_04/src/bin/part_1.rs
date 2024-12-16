use std::collections::HashSet;
use std::fs;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, PartialEq, Debug)]
enum Direction {
    Up,
    UpRight,
    UpLeft,
    Down,
    DownRight,
    DownLeft,
    Left,
    Right,
}

fn get_next_coordinate(
    x: usize,
    y: usize,
    direction: &Direction,
    max_width: usize,
    max_height: usize,
) -> Option<(usize, usize)> {
    match direction {
        Direction::Up => {
            if y == 0 {
                return None;
            }
            Some((x, y - 1))
        }
        Direction::UpRight => {
            if y == 0 || x >= max_width - 1 {
                return None;
            }
            Some((x + 1, y - 1))
        }
        Direction::UpLeft => {
            if x == 0 || y == 0 {
                return None;
            }
            Some((x - 1, y - 1))
        }
        Direction::Down => {
            if y >= max_height - 1 {
                return None;
            }
            Some((x, y + 1))
        }
        Direction::DownRight => {
            if x >= max_width - 1 || y >= max_height - 1 {
                return None;
            }
            Some((x + 1, y + 1))
        }
        Direction::DownLeft => {
            if x == 0 || y >= max_height - 1 {
                return None;
            }
            Some((x - 1, y + 1))
        }
        Direction::Left => {
            if x == 0 {
                return None;
            }
            Some((x - 1, y))
        }
        Direction::Right => {
            if x >= max_width - 1 {
                return None;
            }
            Some((x + 1, y))
        }
    }
}

fn traverse_word(x: usize, y: usize, input: Vec<Vec<char>>) -> Option<Vec<Vec<(usize, usize)>>> {
    let mut coordinates = vec![];

    println!("Traversing char {} at {:?}, {:?}", input[y][x], x, y);

    // Traverse in every direction in a straight line and continue to find the full word "XMAS"
    for direction in Direction::iter() {
        dbg!(&direction);
        if direction == Direction::Up
            || direction == Direction::UpLeft
            || direction == Direction::UpRight
        {
            if y < 3 {
                dbg!("Skipping Up");
                continue;
            }
        }
        if direction == Direction::Down
            || direction == Direction::DownLeft
            || direction == Direction::DownRight
        {
            if y > input.len() - 4 {
                dbg!("Skipping Down");
                continue;
            }
        }
        if direction == Direction::Left
            || direction == Direction::UpLeft
            || direction == Direction::DownLeft
        {
            if x < 3 {
                dbg!("Skipping Left");
                continue;
            }
        }
        if direction == Direction::Right
            || direction == Direction::UpRight
            || direction == Direction::DownRight
        {
            if x > input[0].len() - 4 {
                dbg!("Skipping Right");
                continue;
            }
        }

        dbg!("Traversing");

        let mut direction_coordinates = vec![(x, y)];
        let mut current_x = x;
        let mut current_y = y;

        let mut index = 0;
        while direction_coordinates.len() < 4 {
            let next_coordinate = get_next_coordinate(
                current_x,
                current_y,
                &direction,
                input[0].len(),
                input.len(),
            );
            if next_coordinate.is_none() {
                break;
            }

            let next_coordinate = next_coordinate.unwrap();
            dbg!(&next_coordinate);

            let next_char = input[next_coordinate.1][next_coordinate.0];
            dbg!(&next_char);
            if index == 0 && next_char == 'M' {
                println!("Adding M");
                direction_coordinates.push(next_coordinate);
                current_x = next_coordinate.0;
                current_y = next_coordinate.1;
            } else if index == 1 && next_char == 'A' {
                println!("Adding A");
                direction_coordinates.push(next_coordinate);
                current_x = next_coordinate.0;
                current_y = next_coordinate.1;
            } else if index == 2 && next_char == 'S' {
                println!("Adding S");
                direction_coordinates.push(next_coordinate);
                current_x = next_coordinate.0;
                current_y = next_coordinate.1;
            } else {
                println!("Breaking at {} @ {:?}", next_char, next_coordinate);
                break;
            }

            index += 1;
        }

        if direction_coordinates.len() == 4 {
            coordinates.push(direction_coordinates);
        }
    }

    Some(coordinates)
}

fn solve_challenge(input: &str) -> u32 {
    /*
       Trivial solution?:
       - Split the input into lines
       - In each line, find the first X, then traverse in every direction and continue to find the full word "XMAS"
       - If the word is found, add the coordinates to a Vector Tuple in a set
       - Return the size of the set
    */

    let mut xmas_coordinates: HashSet<Vec<(usize, usize)>> = HashSet::new();
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    for (y, line) in map.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == 'X' {
                let coordinates = traverse_word(x, y, map.clone());
                // dbg!(&coordinates);

                if let Some(coordinates) = coordinates {
                    // dbg!(&coordinates);
                    xmas_coordinates.extend(coordinates);
                }
            }
        }
    }

    // dbg!(&xmas_coordinates);

    xmas_coordinates.len() as u32
}

fn main() {
    let input = fs::read_to_string("./src/bin/input.txt").unwrap();

    println!("Answer: {}", solve_challenge(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX";
        assert_eq!(solve_challenge(input), 18)
    }
}
