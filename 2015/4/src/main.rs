use md5;

fn correct_hash(hash: &str, zeroes: i32) -> bool {
    let prefix = (0..zeroes).map(|_| '0').collect::<String>();
    hash.starts_with(prefix.as_str())
}

fn makes_correct_hash(key: &str, number: i32, zeroes: i32) -> bool {
    let combined = format!("{}{}", key, number);
    let digest = md5::compute(combined);
    let hash = format!("{:x}", digest);
    correct_hash(&hash, zeroes)
}

fn guess_answer(key: &str, zeroes: i32) -> i32 {
    let mut i = 0;
    while !makes_correct_hash(key, i, zeroes) {
        i += 1;
    }
    i
}

fn main() {
    let key = "bgvyzdsv";
    println!("{}", guess_answer(key, 5));
    println!("{}", guess_answer(key, 6));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guess_hash() {
        assert_eq!(guess_answer("abcdef", 5), 609043);
        assert_eq!(guess_answer("pqrstuv", 5), 1048970);
    }

    #[test]
    fn test_correct_hash() {
        assert_eq!(correct_hash("000001dbbfa", 5), true);
        assert_eq!(correct_hash("000006136ef", 5), true);
    }

    #[test]
    fn test_makes_correct_hash() {
        assert_eq!(makes_correct_hash("abcdef", 609043, 5), true);
        assert_eq!(makes_correct_hash("pqrstuv", 1048970, 5), true);
    }
}
