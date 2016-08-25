pub fn add(num_l: &str, num_r: &str) -> String {
    let sum = String::from(num_l) + num_r;

    compress(&sum)
}

pub fn compress(num: &str) -> String {
    if num == "IIIII" {
        "V".to_string()
    } else {
        num.to_string()
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
    fn compress_iiiii() {
        assert_eq!("V", compress("IIIII"));
    }
}
