use std::fs;
use std::ops::Range;

fn solve_challenge(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mul_indeces = line.match_indices("mul(").collect::<Vec<_>>();
            dbg!(&mul_indeces);

            /* Unavailable Ranges:
             * How to get the unavailable ranges:
             * 1. Find all the don't() and do() indices
             * 2. For each don't(), find the next do() that is larger than it
             * 3. Add the range to the unavailable ranges
             */
            let unavailable_ranges: Vec<Range<usize>> = line
                // Find all the don't() indices
                .match_indices("don't()")
                // Find the next do() that is larger than it or use line length
                .map(|dont_index| {
                    match line
                        .match_indices("do()")
                        .find(|do_index| do_index.0 > dont_index.0)
                    {
                        Some(do_index) => dont_index.0..do_index.0,
                        None => dont_index.0..line.len(),
                    }
                })
                .collect();
            dbg!(&unavailable_ranges);

            let mut matches: Vec<u32> = vec![];

            let max_matches = mul_indeces.len();
            let mut current_match = 0;
            let mut index = mul_indeces[current_match].0 + 4;

            // Numbers can be 1, 2 or 3 digits
            let mut first_number = String::new();
            let mut first_number_finished = false;
            let mut second_number = String::new();

            while index < line.chars().count() {
                if unavailable_ranges
                    .iter()
                    .any(|range| range.contains(&index))
                {
                    // dbg!("Bad Range");
                    if current_match + 1 < max_matches {
                        current_match += 1;
                        index = mul_indeces[current_match].0 + 4;
                        continue;
                    } else {
                        dbg!("No more matches");
                        break;
                    }
                }

                dbg!(&line.chars().nth(index).unwrap());
                dbg!(&index);
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
                        dbg!(
                            "Invalid character for first number: {}",
                            // line.chars().nth(index).unwrap()
                        );
                        dbg!(line.chars().nth(index).unwrap());
                        first_number = String::new();
                        first_number_finished = false;
                        if current_match + 1 <= max_matches {
                            current_match += 1;
                            index = mul_indeces[current_match].0 + 3;
                        } else {
                            break;
                        }
                    }
                } else {
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

                        if current_match + 1 < max_matches {
                            current_match += 1;
                            index = mul_indeces[current_match].0 + 3;
                        } else {
                            break;
                        }
                    } else {
                        first_number = String::new();
                        first_number_finished = false;
                        second_number = String::new();

                        if current_match + 1 < max_matches {
                            current_match += 1;
                            index = mul_indeces[current_match].0 + 3;
                        } else {
                            break;
                        }
                    }
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

    // TOO LOW : 49416743
    // TOO HIGH: 99812796
    // TOO HIGH: 102489528
    // TOO HIGH: 104804964

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
}
