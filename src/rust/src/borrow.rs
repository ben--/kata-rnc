pub fn borrow(num: &str, digit: char) -> Result<String, &'static str> {
    match digit {
        'V' => {
            Ok(num.to_string().replace("X", "VV"))
        },
        'I' => {
            Ok(num.replace("V", "IIIII"))
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
}
