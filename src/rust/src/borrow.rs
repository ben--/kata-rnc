use std::cmp::Ordering;

use cmp;

pub fn borrow(num: &str, needed: char) -> Result<String, String> {
    match num.rfind(|c| cmp(c, needed) != Ordering::Less) {
        Some(i) => {
            let (prefix, end) = num.split_at(i);
            let (expand_char, suffix) = end.split_at(1);
            match expand_digit_to_make(expand_char.to_string(), needed) {
                Ok(middle) => Ok(prefix.to_string() + &middle + suffix),
                Err(e) => Err(e),
            }
        },
        None => {
            Err("Would be a negative number".into())
        }
    }
}

fn expand_digit_to_make(mut original: String, needed: char) -> Result<String, String> {
    match original.pop() {
        Some(c) if c == needed => {
            original.push(c);
            Ok(original)
        },
        Some('M') => expand_digit_to_make(original + "DD", needed),
        Some('D') => expand_digit_to_make(original + "CCCCC", needed),
        Some('C') => expand_digit_to_make(original + "LL", needed),
        Some('L') => expand_digit_to_make(original + "XXXXX", needed),
        Some('X') => expand_digit_to_make(original + "VV", needed),
        Some('V') => expand_digit_to_make(original + "IIIII", needed),
        _ => Err(format!("Don't know how to borrow from {}", original))
    }
}

#[cfg(test)]
mod tests {
    use super::borrow;

    #[test]
    fn cannot_get_something_from_nothing() {
        assert!(borrow("", 'I').is_err());
    }

    #[test]
    fn borrow_i_from_ii_returns_string_without_change() {
        assert_eq!("II", borrow("II", 'I').unwrap());
    }

    #[test]
    fn borrow_i_from_v() {
        assert_eq!("IIIII", borrow("V", 'I').unwrap());
    }

    #[test]
    fn borrow_v_from_x() {
        assert_eq!("VV", borrow("X", 'V').unwrap());
    }

    #[test]
    fn borrow_v_from_cx() {
        assert_eq!("CVV", borrow("CX", 'V').unwrap());
    }

    #[test]
    fn borrow_i_from_x_borrows_transitively() {
        assert_eq!("VIIIII", borrow("X", 'I').unwrap());
    }

    #[test]
    fn borrow_v_from_xx_only_borrows_once() {
        assert_eq!("XVV", borrow("XX", 'V').unwrap());
    }

    #[test]
    fn borrow_v_from_cxi_returns_the_tail_part_properly() {
        assert_eq!("CVVI", borrow("CXI", 'V').unwrap());
    }

    #[test]
    fn borrow_x_from_l() {
        assert_eq!("XXXXX", borrow("L", 'X').unwrap());
    }

    #[test]
    fn borrow_l_from_c() {
        assert_eq!("LL", borrow("C", 'L').unwrap());
    }

    #[test]
    fn borrow_c_from_d() {
        assert_eq!("CCCCC", borrow("D", 'C').unwrap());
    }

    #[test]
    fn borrow_d_from_m() {
        assert_eq!("DD", borrow("M", 'D').unwrap());
    }
}
