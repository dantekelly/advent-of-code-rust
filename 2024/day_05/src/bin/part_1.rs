use std::collections::HashSet;
use std::fs;

fn solve_challenge(input: &str) -> u32 {
    let (ordering_rule_lines, page_lines) = input.split_once("\n\n").unwrap();

    let ordering_rules: Vec<(u32, u32)> = ordering_rule_lines
        .lines()
        .map(|line| {
            let (a, b) = line
                .split_once("|")
                .unwrap();

            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect();
    let ordering_rules_set: HashSet<u32> = ordering_rules.iter().flat_map(|(a, b)| [*a, *b]).collect();

    page_lines
        .lines()
        .map(|line| {
            let pages: Vec<u32> = line.split(",").map(|page| page.parse().unwrap()).collect();

            match pages
                .iter()
                .all(|page| {
                    if ordering_rules_set.contains(page) {
                        return ordering_rules
                            .iter()
                            .filter(|(a, b)| *a == *page || *b == *page)
                            .all(|(rule_a, rule_b)| {
                                let rule_a_index = pages.iter().position(|position| position == rule_a);
                                let rule_b_index = pages.iter().position(|position| position == rule_b);

                                match (rule_a_index, rule_b_index) {
                                    (Some(rule_a_index), Some(rule_b_index)) => {
                                        rule_a_index < rule_b_index
                                    }
                                    _ => {
                                        true
                                    }
                                }
                            });
                    }

                    true
                }) {
                    true => pages[(pages.len() as f32 / 2.0).floor() as usize],
                    false => 0
                }
        })
        .sum()

    // result
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
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";
        assert_eq!(solve_challenge(input), 143)
    }
}
