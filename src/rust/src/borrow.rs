pub fn borrow(num: &str, digit: char) -> Result<String, &'static str> {
    match digit {
        'V' => {
            let mut parts: Vec<&str> = num.rsplitn(2, "X").collect();
            parts.reverse();
            Ok(parts.join("VV"))
        },
        'I' => {
            Ok(num.replace("X", "VV").replace("V", "IIIII"))
        },
        _ => {
            Ok(num.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::borrow;

    #[test]
    fn cannot_borrow_i_from_ii() {
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
}
