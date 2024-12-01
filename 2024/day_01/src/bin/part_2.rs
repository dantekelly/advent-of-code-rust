use std::fs;

fn solve_challenge(input: String) -> i32 {
    let mut first_col: Vec<i32> = vec![];
    let mut second_col: Vec<i32> = vec![];

    input.lines().for_each(|line| {
        let cols: Vec<&str> = line.split_whitespace().collect();

        first_col.push(cols[0].to_string().parse::<i32>().unwrap());
        second_col.push(cols[1].to_string().parse::<i32>().unwrap());
    });

    let mut index = 0;
    let mut solution = 0;
    while index < first_col.len() {
        let first_item = first_col[index];
        let match_count = second_col
            .iter()
            .filter(|&item| *item == first_item)
            .count();

        solution += first_item * match_count as i32;
        index += 1;
    }

    return solution;
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
