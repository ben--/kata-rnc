use borrow;
use denormalize;
use normalize;

pub fn sub(num_l: &str, num_r: &str) -> Result<String, &'static str> {
    let from = denormalize(num_l);
    match from.find(num_r) {
        Some(begin) => {
            let end = begin + num_r.len();
            Ok(from[end..from.len()].to_string())
        },
        None => {
            let from = borrow(from.as_ref(), 'I').unwrap();
            match from.find(num_r) {
                Some(begin) => {
                    let end = begin + num_r.len();
                    Ok(normalize(from[end..from.len()].as_ref()))
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
    fn sub_iv_i_denormalizes_before_subtracting() {
        assert_eq!("III", sub("IV", "I").unwrap());
    }
}
