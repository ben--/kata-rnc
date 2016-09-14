use borrow;
use denormalize;
use normalize;
use valid;

pub fn sub(num_l: &str, num_r: &str) -> Result<String, String> {
    if !valid(num_l) {
        Err(format!("Invalid numeral {}", num_l))
    } else if !valid(num_r) {
        Err(format!("Invalid numeral {}", num_r))
    } else {
        let lhs = denormalize(num_l);
        let rhs = denormalize(num_r);

        match rhs.chars().fold(Ok(lhs), |from, digit| {
            match from {
                Ok(remaining) => {
                    let parts: Vec<&str> = remaining.splitn(2, digit).collect();
                    if parts.len() == 2 {
                        Ok(parts.join(""))
                    } else {
                        match borrow(remaining.as_ref(), digit) {
                            Ok(expanded) => {
                                borrow(remaining.as_ref(), digit).unwrap();
                                let expanded_parts: Vec<&str> = expanded.splitn(2, digit).collect();
                                Ok(expanded_parts.join(""))
                            },
                            Err(_) => {
                                Err("Could not borrow a letter".into())
                            }
                        }
                    }
                },
                Err(e) => {
                    Err(e)
                }
            }
        }) {
            Ok(denorm) => { Ok(normalize(&denorm).unwrap()) },
            Err(e) => { Err(e) }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::sub;

    #[test]
    fn sub_ii_i() {
        assert_eq!("I", sub("II", "I").unwrap());
    }

    #[test]
    fn sub_iii_i() {
        assert_eq!("II", sub("III", "I").unwrap());
    }

    #[test]
    fn sub_v_ii_borrows_i_from_v() {
        assert_eq!("III", sub("V", "II").unwrap());
    }

    #[test]
    fn sub_v_i_normalizes_result() {
        assert_eq!("IV", sub("V", "I").unwrap());
    }

    #[test]
    fn sub_iv_i_denormalizes_rhs_before_subtracting() {
        assert_eq!("III", sub("IV", "I").unwrap());
    }

    #[test]
    fn sub_v_iv_denormalizes_rhs_before_subtracting() {
        assert_eq!("I", sub("V", "IV").unwrap());
    }

    #[test]
    fn sub_x_v_borrows_v_from_x() {
        assert_eq!("V", sub("X", "V").unwrap());
    }

    #[test]
    fn sub_x_v_borrows_v_from_x_at_end_of_number() {
        assert_eq!("CV", sub("CX", "V").unwrap());
    }

    #[test]
    fn sub_x_i_borrows_i_from_x_transitively() {
        assert_eq!("IX", sub("X", "I").unwrap());
    }

    #[test]
    fn sub_i_ii_returns_an_error() {
        assert!(sub("I", "II").is_err());
    }

    #[test]
    fn sub_m_i_fully_borrow_from_m() {
        assert_eq!("CMXCIX", sub("M", "I").unwrap());
    }

    #[test]
    fn sub_fails_when_lhs_is_invalid() {
        assert!(sub("IJ", "I").is_err());
    }

    #[test]
    fn sub_fails_when_rhs_is_invalid() {
        assert!(sub("MM", "IM").is_err());
    }
}
