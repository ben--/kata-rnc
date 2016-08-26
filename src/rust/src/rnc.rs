pub fn add(num_l: &str, num_r: &str) -> String {
    let mut sum: Vec<u8> = (String::from(num_l) + num_r).into();

    sum.sort();
    sum.reverse();

    compress(&String::from_utf8(sum).unwrap())
}

pub fn compress(num: &str) -> String {
    match num {
        "IIIII" => "V".to_string(),
        "IIII" => "IV".to_string(),
        _ => num.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::add;
    use super::compress;

    #[test]
    fn add_i_i() {
        assert_eq!("II", add("I", "I"));
    }

    #[test]
    fn add_i_ii() {
        assert_eq!("III", add("I", "II"));
    }

    #[test]
    fn add_ii_iii() {
        assert_eq!("V", add("II", "III"));

    }

    #[test]
    fn add_v_i() {
        assert_eq!("VI", add("V", "I"));
    }

    #[test]
    fn add_i_v() {
        assert_eq!("VI", add("I", "V"));
    }

    #[test]
    fn compress_iiiii() {
        assert_eq!("V", compress("IIIII"));
    }

    #[test]
    fn compress_iiii() {
        assert_eq!("IV", compress("IIII"));
    }
}
