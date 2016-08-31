use borrow;
use normalize;

pub fn sub(num_l: &str, num_r: &str) -> Result<String, &'static str> {
    match num_l.find(num_r) {
        Some(begin) => {
            let end = begin + num_r.len();
            Ok(num_l[end..num_l.len()].to_string())
        },
        None => {
            let num_l = borrow(num_l, 'I').unwrap();
            match num_l.find(num_r) {
                Some(begin) => {
                    let end = begin + num_r.len();
                    Ok(normalize(num_l[end..num_l.len()].as_ref()))
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
}
