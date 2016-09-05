use std::cmp::Ordering;

use cmp;

pub fn borrow(num: &str, needed: char) -> Result<String, &'static str> {
    match num.rfind(|c| cmp(c, needed) != Ordering::Less) {
        Some(i) => {
            let (prefix, end) = num.split_at(i);
            let (expand_char, suffix) = end.split_at(1);
            match expand_char {
                "X" => {
                    if needed == 'I' {
                        Ok(prefix.to_string() + "VIIIII")
                    } else {
                        Ok(prefix.to_string() + "VV" + suffix)
                    }
                },
                "V" => {
                    Ok(prefix.to_string() + "IIIII")
                },
                "I" => {
                    Ok(num.into())
                },
                _ => {
                    Err("unknown digit")
                }
            }
        },
        None => {
            Err("Would be a negative number")
        }
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
}
