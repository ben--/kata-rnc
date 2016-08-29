use std::cmp::Ordering;

pub fn add(num_l: &str, num_r: &str) -> String {
    let sum = merge(denormalize(num_l), denormalize(num_r));
    normalize(&sum)
}

fn cmp(l: char, r: char) -> Ordering {
    match(l, r) {
        ('L', 'L') => Ordering::Equal,
        (_, 'L') => Ordering::Less,
        ('L', _) => Ordering::Greater,
        ('X', 'X') => Ordering::Equal,
        (_, 'X') => Ordering::Less,
        ('X', _) => Ordering::Greater,
        ('V', 'V') => Ordering::Equal,
        (_, 'V') => Ordering::Less,
        ('V', _) => Ordering::Greater,
        (_, _) => Ordering::Equal,
    }
}

fn merge(num_l: String, num_r: String) -> String {
    let mut digits_l = num_l.chars();
    let mut digits_r = num_r.chars();

    let mut sum = String::new();

    let mut next_l = digits_l.next();
    let mut next_r = digits_r.next();
    loop {
        match (next_l, next_r) {
            (Some(l), Some(r)) => {
                if cmp(l, r) == Ordering::Greater {
                    sum.push(l);
                    next_l = digits_l.next();
                } else {
                    sum.push(r);
                    next_r = digits_r.next();
                }
            },
            (Some(l), None) => {
                sum.push(l);
                next_l = digits_l.next();
            },
            (None, Some(r)) => {
                sum.push(r);
                next_r = digits_r.next();
            },
            (None, None) => { break }
        }
    }

    sum
}

pub fn denormalize(num: &str) -> String {
    let num = num.replace("IV", "IIII");
    let num = num.replace("IX", "VIIII");
    let num = num.replace("XL", "XXXX");
    num
}

pub fn normalize(num: &str) -> String {
    let num = num.replace("IIIII", "V");
    let num = num.replace("IIII", "IV");
    let num = num.replace("VIV", "IX");
    let num = num.replace("VV", "X");
    let num = num.replace("XXXXX", "L");
    num
}

#[cfg(test)]
mod tests {
    use super::add;
    use super::cmp;
    use super::normalize;
    use super::denormalize;

    use std::cmp::Ordering;

    #[test]
    fn i_equal_to_i() {
        assert_eq!(Ordering::Equal, cmp('I', 'I'));
    }

    #[test]
    fn i_less_than_v() {
        assert_eq!(Ordering::Less, cmp('I', 'V'));
    }

    #[test]
    fn v_greater_than_i() {
        assert_eq!(Ordering::Greater, cmp('V', 'I'));
    }

    #[test]
    fn v_equal_to_v() {
        assert_eq!(Ordering::Equal, cmp('V', 'V'));
    }

    #[test]
    fn x_greater_than_v() {
        assert_eq!(Ordering::Greater, cmp('X', 'V'));
    }

    #[test]
    fn v_less_than_x() {
        assert_eq!(Ordering::Less, cmp('V', 'X'));
    }

    #[test]
    fn x_equal_to_x() {
        assert_eq!(Ordering::Equal, cmp('X', 'X'));
    }

    #[test]
    fn l_greater_than_x() {
        assert_eq!(Ordering::Greater, cmp('L', 'X'));
    }

    #[test]
    fn x_less_than_l() {
        assert_eq!(Ordering::Less, cmp('X', 'L'));
    }

    #[test]
    fn l_equal_to_l() {
        assert_eq!(Ordering::Equal, cmp('L', 'L'));
    }

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
    fn add_l_i_supports_l() {
        assert_eq!("LI", add("L", "I"));
    }

    #[test]
    fn add_l_xi_understands_l_x_sort_order() {
        assert_eq!("LXI", add("L", "XI"));
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
    fn denormalize_xl() {
        assert_eq!("XXXX", denormalize("XL"));
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

    #[test]
    fn normalize_xxxxx() {
        assert_eq!("L", normalize("XXXXX"));
    }
}
