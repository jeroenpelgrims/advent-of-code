use std::fs;

fn parse_line(line: &str) -> Vec<i32> {
    line.split(" ")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

fn all_inc_or_dec(xs: &Vec<i32>) -> bool {
    let pairs: Vec<_> = xs.windows(2).collect();
    let all_inc = pairs.iter().all(|pair| pair[0] < pair[1]);
    let all_dec = pairs.iter().all(|pair| pair[0] > pair[1]);
    all_inc || all_dec
}

fn diff_1_to_3(xs: &Vec<i32>) -> bool {
    let pairs: Vec<_> = xs.windows(2).collect();
    pairs.iter().all(|pair| {
        let diff = (pair[0] - pair[1]).abs();
        diff >= 1 && diff <= 3
    })
}

fn is_safe(xs: &Vec<i32>) -> bool {
    all_inc_or_dec(xs) && diff_1_to_3(xs)
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let lines: Vec<_> = input.lines().into_iter().map(parse_line).collect();
    let safe_count = lines.iter().filter(|line| is_safe(line)).count();
    println!("1: {:?}", safe_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {}
}
