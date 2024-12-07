use std::fs;

fn solve_challenge(input: String) -> i32 {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|level| level.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|levels| {
            for (i, _) in levels.into_iter().enumerate() {
                let mut curr_levels = levels.clone();
                curr_levels.remove(i);

                let is_ascending = curr_levels.windows(2).all(|w| w[0] < w[1]);
                let is_descending = curr_levels.windows(2).all(|w| w[0] > w[1]);

                if (is_ascending || is_descending)
                    && curr_levels.windows(2).all(|w| {
                        let diff = w[0].abs_diff(w[1]);
                        diff >= 1 && diff <= 3
                    })
                {
                    return true;
                }
            }

            false
        })
        .count() as i32
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
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
            .to_string();

        assert_eq!(solve_challenge(input), 4)
    }
}
