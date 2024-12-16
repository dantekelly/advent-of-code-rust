use std::fs;
use strum_macros::EnumIter;

#[derive(EnumIter, PartialEq, Debug)]
enum Direction {
    UpLeft,
    DownRight,
    UpRight,
    DownLeft,
}

fn get_next_coordinate(
    x: usize,
    y: usize,
    direction: &Direction,
    max_width: usize,
    max_height: usize,
) -> Option<(usize, usize)> {
    match direction {
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
    }
}

fn traverse_word(x: usize, y: usize, input: Vec<Vec<char>>) -> Result<(), ()> {
    let mut coordinates = vec![];
    let directions = [
        (Direction::UpLeft, Direction::DownRight),
        (Direction::UpRight, Direction::DownLeft),
    ];

    /* Intention:
    - Traverse each direction tuple
    - If the directions are out of bounds, skip the direction
    - Check if the direction.1 and direction.2 are not equal and they are either "M" or "S"
    - If they are, add the coordinates to a Vector Tuple in a set
    - Return the size of the set
    */
    for (dir1, dir2) in directions {
        if y == 0 || y == input.len() - 1 || x == 0 || x == input[0].len() - 1 {
            continue;
        }

        let first_coords = get_next_coordinate(x, y, &dir1, input[0].len(), input.len()).unwrap();
        let second_coords = get_next_coordinate(x, y, &dir2, input[0].len(), input.len()).unwrap();

        let first_char = input[first_coords.1][first_coords.0];
        let second_char = input[second_coords.1][second_coords.0];

        if first_char == second_char {
            continue;
        }

        if (first_char == 'M' || first_char == 'S') && (second_char == 'M' || second_char == 'S') {
            coordinates.push(vec![first_coords, second_coords]);
        }
    }

    match coordinates.len() {
        2 => Ok(()),
        _ => Err(()),
    }
}

fn solve_challenge(input: &str) -> u32 {
    /*
       Trivial solution?:
       - Split the input into lines
       - In each line, find the first X, then traverse in every direction and continue to find the full word "XMAS"
       - If the word is found, add the coordinates to a Vector Tuple in a set
       - Return the size of the set
    */

    let mut count = 0;
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    map.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, c)| {
            if *c == 'A' {
                if let Ok(()) = traverse_word(x, y, map.clone()) {
                    count += 1;
                }
            }
        });
    });

    count as u32
}

fn main() {
    let input = fs::read_to_string("./src/bin/input.txt").unwrap();

    // Correct Answer: 1835
    println!("Answer: {}", solve_challenge(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";
        assert_eq!(solve_challenge(input), 9)
    }
}
