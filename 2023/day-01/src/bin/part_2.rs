// Regex is a good solution, but it felt way too easy.
use std::fs;

fn is_number_word_at_position(s: &str, pos: usize) -> Option<char> {
    let number_words = [
        ("one", '1'), ("two", '2'), ("three", '3'), ("four", '4'),
        ("five", '5'), ("six", '6'), ("seven", '7'), ("eight", '8'), ("nine", '9')
    ];

    for (word, digit) in number_words {
        if pos + word.len() <= s.len() && &s[pos..pos + word.len()] == word {
            return Some(digit);
        }
    }
    None
}

fn solve_challenge(input: String) -> i32 {
    input
        .lines()
        .map(|row| {
            let mut digits = Vec::new();
            let chars: Vec<char> = row.chars().collect();

            for i in 0..row.len() {
                if chars[i].is_digit(10) {
                    digits.push(chars[i]);
                } else if let Some(digit) = is_number_word_at_position(row, i) {
                    digits.push(digit);
                }
            }

            let first = digits.first().unwrap_or(&'0');
            let last = digits.last().unwrap_or(first);

            format!("{}{}", first, last).parse::<i32>().unwrap()
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
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen".to_string();

        assert_eq!(solve_challenge(input), 281);
    }
}