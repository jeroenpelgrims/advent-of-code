use std::fs;

type BatteryBank = Vec<u8>;

fn parse_line(line: &str) -> BatteryBank {
    line.chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect()
}

fn parse_input(input: &str) -> Vec<BatteryBank> {
    input.lines().map(parse_line).collect()
}

/*
Look at the leftmost digits. (there need to be at least n digits left to the right)
Take the highest digit among them. Store this together with the index in the result.
Repeat, starting from the index after the chosen digit, until n digits have been chosen.
(But this time there only need to be n - result.len() digits left to the right)
 */
fn max_joltage(bank: &BatteryBank, n: usize) -> u64 {
    let mut result: Vec<(usize, u8)> = vec![];
    let cells_with_indices = bank.into_iter().enumerate().collect::<Vec<_>>();

    while result.len() < n {
        // Must be behind the last chosen digit
        let candidate_range_start =
            result.last().map(|item| item.0 + 1).unwrap_or(0);
        // Must leave enough digits to the right
        let candidate_range_end = bank.len() - (n - result.len() - 1);
        let candidate_range =
            &cells_with_indices[candidate_range_start..candidate_range_end];

        let highest_candidate = candidate_range
            .iter()
            .fold(None, |max, item| match max {
                None => Some(item.clone()),
                Some((_, &max_value)) if item.1 > &max_value => {
                    Some(item.clone())
                }
                Some(current_max) => Some(current_max),
            })
            .unwrap();

        result.push((highest_candidate.0, highest_candidate.1.clone()));
    }

    result
        .iter()
        .map(|(_, digit)| *digit as u64)
        .fold(0, |acc, d| acc * 10 + d)
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let banks = parse_input(&input);

    println!(
        "part 1: {:?}",
        banks.iter().map(|bank| max_joltage(bank, 2)).sum::<u64>()
    );
    println!(
        "part 2: {:?}",
        banks.iter().map(|bank| max_joltage(bank, 12)).sum::<u64>()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    mod max_joltage {
        use super::*;

        #[test]
        fn case_2() {
            let bank = parse_line("987654321111111");
            assert_eq!(max_joltage(&bank, 2), 98);
            let bank = parse_line("811111111111119");
            assert_eq!(max_joltage(&bank, 2), 89);
            let bank = parse_line("234234234234278");
            assert_eq!(max_joltage(&bank, 2), 78);
            let bank = parse_line("818181911112111");
            assert_eq!(max_joltage(&bank, 2), 92);
        }

        #[test]
        fn case_12() {
            let bank = parse_line("987654321111111");
            assert_eq!(max_joltage(&bank, 12), 987654321111);
            let bank = parse_line("811111111111119");
            assert_eq!(max_joltage(&bank, 12), 811111111119);
            let bank = parse_line("234234234234278");
            assert_eq!(max_joltage(&bank, 12), 434234234278);
            let bank = parse_line("818181911112111");
            assert_eq!(max_joltage(&bank, 12), 888911112111);
        }
    }
}
