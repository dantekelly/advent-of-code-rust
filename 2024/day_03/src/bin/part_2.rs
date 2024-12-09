use std::{fs, ops::Range};

fn parse_mul(input: &str, start: usize) -> Option<u32> {
    let mut chars = input[start..].chars();
    let mut first_number = String::new();
    let mut second_number = String::new();

    // Skip mul(
    for _ in 0..4 {
        chars.next();
    }

    for c in chars.by_ref() {
        if c.is_ascii_digit() {
            first_number.push(c);
        } else if c == ',' {
            break;
        } else {
            return None;
        }
    }

    for c in chars.by_ref() {
        if c.is_ascii_digit() {
            second_number.push(c);
        } else if c == ')' {
            break;
        } else {
            return None;
        }
    }

    Some(first_number.parse::<u32>().ok()? * second_number.parse::<u32>().ok()?)
}

fn solve_challenge(input: &str) -> u32 {
    let unavailable_ranges: Vec<Range<usize>> = input
        .match_indices("don't()")
        .map(|(start, _)| {
            match input
                .match_indices("do()")
                .find(|&(do_start, _)| do_start > start)
            {
                Some((end, _)) => start..end,
                None => start..input.len(),
            }
        })
        .collect();

    input
        .match_indices("mul(")
        .filter(|(start, _)| !unavailable_ranges.iter().any(|range| range.contains(start)))
        .filter_map(|(start, _)| parse_mul(input, start))
        .sum()
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
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        assert_eq!(solve_challenge(input), 48)
    }

    #[test]
    fn test_parse_mul() {
        assert_eq!(parse_mul("mul(2,4)", 0), Some(8));
        assert_eq!(parse_mul("mul(2,4)", 1), None);
        assert_eq!(parse_mul("mul[3,7]", 0), None);
        assert_eq!(parse_mul("mul(3,7]", 0), None);
        assert_eq!(parse_mul("mul(3,7)", 0), Some(21));
    }
}
