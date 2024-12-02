use std::fs;

fn parse_line(line: &str) -> Vec<i32> {
    line.split(" ")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

fn is_safe(xs: &Vec<i32>, direction: i32) -> bool {
    let _xs = format!("{:?}", xs);
    match xs.as_slice() {
        [] | [_] => true,
        [a, b, rest @ ..] => {
            let diff = (a - b).abs();
            let diff_ok = diff >= 1 && diff <= 3;
            let new_direction = (b - a).min(1).max(-1);
            let inc_dec_ok = direction == new_direction || direction == 0;

            if diff_ok && inc_dec_ok {
                let mut xss = vec![*b];
                xss.extend_from_slice(rest);
                is_safe(&xss, new_direction)
            } else {
                false
            }
        }
    }
}

fn is_safe_tolerate1(xs: &Vec<i32>) -> bool {
    if is_safe(xs, 0) {
        return true;
    }

    for index in 0..(xs.len() - 1) {
        let xs_removed: Vec<i32> = xs
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != index)
            .map(|(_, &val)| val)
            .collect();
        if is_safe(&xs_removed, 0) {
            return true;
        }
    }
    return false;
}

fn main() {
    let input = fs::read_to_string("./test-input.txt").unwrap();
    let lines: Vec<_> = input.lines().into_iter().map(parse_line).collect();
    let safe_count1 = lines.iter().filter(|line| is_safe(line, 0)).count();
    println!("1: {:?}", safe_count1);
    let safe_count2 =
        lines.iter().filter(|line| is_safe_tolerate1(line)).count();
    println!("2: {:?}", safe_count2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_safe() {
        assert!(is_safe(&vec![7, 6, 4, 2, 1], 0));
        assert!(!is_safe(&vec![1, 2, 7, 8, 9], 0));
        assert!(!is_safe(&vec![9, 7, 6, 2, 1], 0));
        assert!(!is_safe(&vec![1, 3, 2, 4, 5], 0));
        assert!(!is_safe(&vec![8, 6, 4, 4, 1], 0));
        assert!(is_safe(&vec![1, 3, 6, 7, 9], 0));
    }

    #[test]
    fn test_is_safe_tolerate1() {
        assert!(is_safe_tolerate1(&vec![7, 6, 4, 2, 1]));
        assert!(!is_safe_tolerate1(&vec![1, 2, 7, 8, 9]));
        assert!(!is_safe_tolerate1(&vec![9, 7, 6, 2, 1]));
        assert!(is_safe_tolerate1(&vec![1, 3, 2, 4, 5]));
        assert!(is_safe_tolerate1(&vec![8, 6, 4, 4, 1]));
        assert!(is_safe_tolerate1(&vec![1, 3, 6, 7, 9]));
    }
}
