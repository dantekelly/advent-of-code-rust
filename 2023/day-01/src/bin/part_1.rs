// Regex is a good solution, but it felt way too easy.
use std::fs;

fn solve_challenge(input: String) -> i32 {
    input
        .lines()
        .map(|row| {
            let mut numbers = row
                .chars()
                .into_iter()
                .filter(|x| x.is_digit(10));

            let first = numbers
                .next()
                .unwrap_or('0')
                .to_string()
                .parse::<i32>()
                .unwrap();
            let last = numbers.next_back();

            match last {
                Some(digit) => first * 10 + digit.to_string().parse::<i32>().unwrap(),
                None => first * 10 + first // If no last digit, use first digit as last
            }
        })
        .sum()
}

fn main() {
    let input = fs::read_to_string("./src/bin/input.txt").unwrap();

    println!("Answer: {}", solve_challenge(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
".to_string();

        assert_eq!(solve_challenge(input), 142);
    }
}