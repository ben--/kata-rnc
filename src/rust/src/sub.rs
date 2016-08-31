use borrow;
use denormalize;
use normalize;

pub fn sub(num_l: &str, num_r: &str) -> Result<String, &'static str> {
    let lhs = denormalize(num_l);
    let rhs = denormalize(num_r);

    match lhs.find(&rhs) {
        Some(begin) => {
            let end = begin + rhs.len();
            Ok(lhs[end..lhs.len()].to_string())
        },
        None => {
            let lhs = borrow(lhs.as_ref(), rhs.chars().next().unwrap()).unwrap();
            match lhs.find(&rhs) {
                Some(begin) => {
                    let end = begin + rhs.len();
                    Ok(normalize(lhs[end..lhs.len()].as_ref()))
                },
                None => {
                    Err("Romans don't know negative numbers")
                }
            }
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
}
