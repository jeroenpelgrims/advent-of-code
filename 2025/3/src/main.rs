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

fn max_joltage(bank: &BatteryBank) -> u32 {
    let largest = bank
        .iter()
        .enumerate()
        .fold(None, |max, item| match max {
            None => Some(item),
            Some(current_max) if item.1 > current_max.1 => Some(item),
            _ => max,
        })
        .unwrap();

    let second_largest_right = bank
        .iter()
        .enumerate()
        .filter(|(index, _)| index > &largest.0)
        .fold(None, |max, item| match max {
            None => Some(item),
            Some(current_max) if item.1 > current_max.1 => Some(item),
            _ => max,
        });

    if let Some(second) = second_largest_right {
        return (*largest.1 as u32) * 10 + (*second.1 as u32);
    }

    let second_largest_left = bank
        .iter()
        .enumerate()
        .filter(|(index, _)| index < &largest.0)
        .fold(None, |max, item| match max {
            None => Some(item),
            Some(current_max) if item.1 > current_max.1 => Some(item),
            _ => max,
        });

    if let Some(second) = second_largest_left {
        return (*second.1 as u32) * 10 + (*largest.1 as u32);
    }

    panic!("Battery bank must have at least two batteries");
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let banks = parse_input(&input);

    println!("{:?}", banks.iter().map(max_joltage).sum::<u32>());
}

#[cfg(test)]
mod tests {
    use super::*;

    mod max_joltage {
        use super::*;

        #[test]
        fn case_1() {
            let bank = parse_line("987654321111111");
            assert_eq!(max_joltage(&bank), 98);
        }

        #[test]
        fn case_2() {
            let bank = parse_line("811111111111119");
            assert_eq!(max_joltage(&bank), 89);
        }

        #[test]
        fn case_3() {
            let bank = parse_line("234234234234278");
            assert_eq!(max_joltage(&bank), 78);
        }

        #[test]
        fn case_4() {
            let bank = parse_line("818181911112111");
            assert_eq!(max_joltage(&bank), 92);
        }
    }
}
