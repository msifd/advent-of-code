use md5::{Md5, Digest};

pub fn hack_pass(input: &str) -> String {
    let mut acc = 0;
    let mut pass = String::new();

    while pass.len() < 8 {
        let mut bytes = Vec::from(input);
        bytes.extend(acc.to_string().as_bytes());

        let mut hasher = Md5::new();
        hasher.update(&bytes);
        let hash = hex::encode(hasher.finalize());

        if hash.chars().take(5).all(|c| c == '0') {
            pass.push(hash.chars().nth(5).unwrap());
        }

        acc += 1;
    }

    pass
}

pub fn hack_second_pass(input: &str) -> String {
    use std::collections::BTreeMap;

    let mut acc = 0;
    let mut pass = BTreeMap::<u8, char>::new();

    while pass.len() < 8 {
        let mut bytes = Vec::from(input);
        bytes.extend(acc.to_string().as_bytes());

        let mut hasher = Md5::new();
        hasher.update(&bytes);
        let hash = hex::encode(hasher.finalize());

        if hash.chars().take(5).all(|c| c == '0') {
            let pos = hash.chars().nth(5).unwrap() as u8 - 48;
            if pos < 8 && !pass.contains_key(&pos) {
                pass.insert(pos, hash.chars().nth(6).unwrap());
            }
        }

        acc += 1;
    }

    pass.iter().map(|(_, c)| *c).collect::<String>()
}

#[test]
fn test_1() {
    assert_eq!("18f47a30".to_string(), hack_pass("abc"));
}

#[test]
fn test_2() {
    assert_eq!("05ace8e3".to_string(), hack_second_pass("abc"));
}