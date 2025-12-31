use itertools::Itertools;
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
    let mut result: Vec<(usize, u64)> = vec![];
    for padding in 0..n {
        let max_index = result.last().map(|item| item.0).unwrap_or(default);
        let candidates = bank.iter().enumerate();
        let candidates = candidates.take(bank.len() - padding);

        let candidates = candidates[0..(bank.len() - padding)];
        let max_candidate = candidates.iter().max().unwrap();
    }
    0
    // let max = [].iter().enumerate().fold(vec![], |acc, | vec![]);
    // 0

    // let combos = bank.iter().combinations(n);
    // println!("combos count: {}", combos.clone().count());
    // let max_first = combos
    //     .clone()
    //     .max_by_key(|combo| combo[0] * 10 + combo[1])
    //     .unwrap();
    // println!("max first two digits: {}{}", max_first[0], max_first[1]);
    // let combos = combos
    //     .filter(|combo| combo[0] == max_first[0] && combo[1] == max_first[1])
    //     .take(10);
    // println!("filtered combos count: {}", combos.clone().count());

    // // let combos = combos.sorted_by(|a, b| Ord::cmp(a[0], b[0])).filter;
    // combos
    //     .map(|combo| {
    //         combo
    //             .iter()
    //             .map(|&&digit| digit as u64)
    //             .fold(0, |acc, d| acc * 10 + d)
    //     })
    //     .max()
    //     .unwrap()
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
