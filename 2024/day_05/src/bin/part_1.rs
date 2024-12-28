use std::collections::HashSet;
use std::fs;

fn solve_challenge(input: &str) -> u32 {
    let lines: Vec<&str> = input.split("\n\n").collect();

    let ordering_rules: Vec<(u32, u32)> = lines[0].lines().map(|line| {
        let (a, b) = line.split_once("|").unwrap();
        (a.parse().unwrap(), b.parse().unwrap())
    }).collect();
    let ordering_rules_set: HashSet<u32> = ordering_rules.iter().flat_map(|(a, b)| [*a, *b]).collect();

    lines[1]
        .lines()
        .map(|line| {
            let pages: Vec<u32> = line.split(",").map(|page| page.parse().unwrap()).collect();

            let is_valid = pages
                .iter()
                .all(|page| {
                    if ordering_rules_set.contains(page) {
                        // Multiple rules can be found for a page, so we need to check them all
                        let rules: Vec<(u32, u32)> = ordering_rules.iter().filter(|(a, b)| *a == *page || *b == *page).cloned().collect();

                        return rules.iter().all(|(rule_a, rule_b)| {
                            let rule_a_index = pages.iter().position(|position| position == rule_a);
                            let rule_b_index = pages.iter().position(|position| position == rule_b);

                            match (rule_a_index, rule_b_index) {
                                (Some(rule_a_index), Some(rule_b_index)) => {
                                    if rule_a_index < rule_b_index {
                                        true
                                    } else {
                                        false
                                    }
                                }
                                (None, Some(_)) => {
                                    true
                                }
                                (Some(_), None) => {
                                    true
                                }
                                (None, None) => {
                                    true
                                }
                            }
                        });
                    }

                    true
                });

            if is_valid {
                let middle_index = (pages.len() as f32 / 2.0).floor() as usize;
                // Collect and return middle number
                let middle_number = pages[middle_index];
                return middle_number;
            }

            0
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
