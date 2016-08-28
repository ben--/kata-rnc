pub fn add(num_l: &str, num_r: &str) -> String {
    let mut sum: Vec<u8> = denormalize(num_l).into();
    sum.extend(denormalize(num_r).bytes());

    sum.sort();
    sum.reverse();

    normalize(&String::from_utf8(sum).unwrap())
}

pub fn denormalize(num: &str) -> String {
    let num = num.replace("IV", "IIII");
    let num = num.replace("IX", "VIIII");
    num
}

pub fn normalize(num: &str) -> String {
    let num = num.replace("IIIII", "V");
    let num = num.replace("IIII", "IV");
    let num = num.replace("VIV", "IX");
    let num = num.replace("VV", "X");
    num
}

#[cfg(test)]
mod tests {
    use super::add;
    use super::normalize;
    use super::denormalize;

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
    fn denormalize_iv() {
        assert_eq!("IIII", denormalize("IV"));
    }

    #[test]
    fn denormalize_ix() {
        assert_eq!("VIIII", denormalize("IX"));
    }

    #[test]
    fn denormalize_xiv_performs_a_partial_denormalization_on_tail() {
        assert_eq!("XIIII", denormalize("XIV"));
    }

    #[test]
    fn normalize_iiiii() {
        assert_eq!("V", normalize("IIIII"));
    }

    #[test]
    fn normalize_iiii() {
        assert_eq!("IV", normalize("IIII"));
    }

    #[test]
    fn normalize_vv() {
        assert_eq!("X", normalize("VV"));
    }

    #[test]
    fn normalize_viiii() {
        assert_eq!("IX", normalize("VIIII"));
    }

    #[test]
    fn normalize_viiiii() {
        assert_eq!("X", normalize("VIIIII"));
    }

}
