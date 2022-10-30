use md5;

fn correct_hash(hash: &str) -> bool {
    hash.starts_with("00000")
}

fn makes_correct_hash(key: &str, number: i32) -> bool {
    let combined = format!("{}{}", key, number);
    let digest = md5::compute(combined);
    let hash = format!("{:x}", digest);
    correct_hash(&hash)
}

fn guess_answer(key: &str) -> i32 {
    let mut i = 0;
    while !makes_correct_hash(key, i) {
        i += 1;
    }
    i
}

fn main() {
    let key = "bgvyzdsv";
    println!("{}", guess_answer(key));
}

#[cfg(test)]
mod tests {
    use md5::compute;

    use super::*;

    #[test]
    fn test_guess_hash() {
        assert_eq!(guess_answer("abcdef"), 609043);
        assert_eq!(guess_answer("pqrstuv"), 1048970);
    }

    #[test]
    fn test_correct_hash() {
        assert_eq!(correct_hash("000001dbbfa"), true);
        assert_eq!(correct_hash("000006136ef"), true);
    }

    #[test]
    fn test_makes_correct_hash() {
        assert_eq!(makes_correct_hash("abcdef", 609043), true);
        assert_eq!(makes_correct_hash("pqrstuv", 1048970), true);
    }
}
