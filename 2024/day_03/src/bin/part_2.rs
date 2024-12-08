use regex::Regex;
use std::fs;

fn solve_challenge(input: &str) -> u32 {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    input
        .lines()
        .map(|line| {
            let mul_indeces = line.match_indices("mul(").collect::<Vec<_>>();
            let mut matches: Vec<u32> = vec![];

            let mut max_matches = mul_indeces.len();
            let mut current_match = 0;
            let mut index = mul_indeces[current_match].0 + 4;

            // Numbers can be 1, 2 or 3 digits
            let mut first_number = String::new();
            let mut first_number_finished = false;
            let mut second_number = String::new();
            let mut second_number_finished = false;

            while index < line.chars().count() {
                if !first_number_finished {
                    if line.chars().nth(index).unwrap().is_digit(10) {
                        first_number = format!(
                            "{}{}",
                            first_number,
                            line.chars().nth(index).unwrap().to_string()
                        );
                    } else if line.chars().nth(index).unwrap() == ',' {
                        first_number_finished = true;
                        dbg!("Found comma, first number is finished");
                    } else {
                        // GOTO Next Match
                        dbg!(
                            "Invalid character for first number: {}",
                            line.chars().nth(index).unwrap()
                        );
                        first_number = String::new();
                        first_number_finished = false;
                        if current_match + 1 < max_matches {
                            current_match += 1;
                            index = mul_indeces[current_match].0 + 3;
                        } else {
                            break;
                        }
                    }
                } else if !second_number_finished {
                    if line.chars().nth(index).unwrap().is_digit(10) {
                        second_number = format!(
                            "{}{}",
                            second_number,
                            line.chars().nth(index).unwrap().to_string()
                        );
                    } else if line.chars().nth(index).unwrap() == ')' {
                        dbg!("Found closing parenthesis, second number is finished");
                        dbg!(&first_number);
                        dbg!(&second_number);
                        matches.push(
                            first_number.parse::<u32>().unwrap_or_default()
                                * second_number.parse::<u32>().unwrap_or_default(),
                        );
                        first_number = String::new();
                        first_number_finished = false;
                        second_number = String::new();
                        second_number_finished = false;

                        if current_match + 1 < max_matches {
                            current_match += 1;
                            index = mul_indeces[current_match].0 + 3;
                        } else {
                            break;
                        }
                    } else {
                        // GOTO Next Match
                        dbg!(
                            "Invalid character for second number: {}",
                            line.chars().nth(index).unwrap()
                        );
                        first_number = String::new();
                        first_number_finished = false;
                        second_number = String::new();
                        second_number_finished = false;
                        if current_match + 1 < max_matches {
                            current_match += 1;
                            index = mul_indeces[current_match].0 + 3;
                        } else {
                            break;
                        }
                    }
                } else {
                    dbg!("Invalid State");
                }
                index += 1;
            }

            dbg!(&matches);
            matches.into_iter().sum::<u32>()
        })
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
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        assert_eq!(solve_challenge(input), 161)
    }
}
