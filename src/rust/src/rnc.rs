pub fn add(num_l: &str, num_r: &str) -> String {
    let mut sum = denormalize(num_l);
    sum.extend(denormalize(num_r));

    sum.sort();
    sum.reverse();

    normalize(&String::from_utf8(sum).unwrap())
}

pub fn denormalize(num: &str) -> Vec<u8> {
    match num {
        "IV" => "IIII".into(),
        _ => num.into(),
    }
}

pub fn normalize(num: &str) -> String {
    match num {
        "IIIII" => "V".to_string(),
        "IIII" => "IV".to_string(),
        _ => num.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::add;
    use super::normalize;

    #[test]
    fn add_i_i() {
        assert_eq!("II", add("I", "I"));
    }

    #[test]
    fn add_i_ii() {
        assert_eq!("III", add("I", "II"));
    }

    #[test]
    fn add_ii_iii_requires_normalization_to_v() {
        assert_eq!("V", add("II", "III"));

    }

    #[test]
    fn add_v_i() {
        assert_eq!("VI", add("V", "I"));
    }

    #[test]
    fn add_i_v_understands_the_relative_order_of_v_and_i() {
        assert_eq!("VI", add("I", "V"));
    }

    #[test]
    fn add_i_iv_denormalizes_before_adding() {
        assert_eq!("V", add("I", "IV"));
    }

    #[test]
    fn normalize_iiiii() {
        assert_eq!("V", normalize("IIIII"));
    }

    #[test]
    fn normalize_iiii() {
        assert_eq!("IV", normalize("IIII"));
    }
}
