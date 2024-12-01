use std::fs;

fn solve_challenge(input: String) -> i32 {
    let (first_col, second_col): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| {
            let mut cols = line
                .split_whitespace()
                .map(|col| col.to_string().parse::<i32>().unwrap());

            (cols.next().unwrap(), cols.next().unwrap())
        })
        .unzip();

    first_col
        .iter()
        .zip(second_col.iter())
        .map(|(first_item, _)| {
            let match_count: i32 =
                second_col.iter().filter(|&item| item == first_item).count() as i32;

            first_item * match_count
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
        let input = "3   4
4   3
2   5
1   3
3   9
3   3"
            .to_string();

        assert_eq!(solve_challenge(input), 31)
    }
}
