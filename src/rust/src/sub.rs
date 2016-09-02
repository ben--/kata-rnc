use borrow;
use denormalize;
use normalize;

pub fn sub(num_l: &str, num_r: &str) -> Result<String, &'static str> {
    let lhs = denormalize(num_l);
    let rhs = denormalize(num_r);

    match rhs.chars().fold(Ok(lhs), |from, digit| {
        match from {
            Ok(remaining) => {
                let parts: Vec<&str> = remaining.splitn(2, digit).collect();
                if parts.len() == 2 {
                    Ok(parts.join(""))
                } else {
                    let expanded = borrow(remaining.as_ref(), digit).unwrap();
                    let expanded_parts: Vec<&str> = expanded.splitn(2, digit).collect();
                    if expanded_parts.len() == 2 {
                        Ok(expanded_parts.join(""))
                    } else {
                        Err("Could not borrow a letter")
                    }
                }
            },
            Err(e) => {
                Err(e)
            }
        }
    }) {
        Ok(denorm) => { Ok(normalize(&denorm)) },
        Err(e) => { Err(e) }
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
}