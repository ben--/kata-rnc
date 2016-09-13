use std::cmp::Ordering;

use cmp;
use denormalize;
use normalize;

pub fn add(num_l: &str, num_r: &str) -> Result<String, String> {
    let sum = merge(denormalize(num_l), denormalize(num_r));
    normalize(&sum)
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

#[cfg(test)]
mod tests {
    use super::add;

    #[test]
    fn add_i_i() {
        assert_eq!("II", add("I", "I").unwrap());
    }

    #[test]
    fn add_i_ii() {
        assert_eq!("III", add("I", "II").unwrap());
    }

    #[test]
    fn add_ii_iii_requires_normalization_to_v() {
        assert_eq!("V", add("II", "III").unwrap());

    }

    #[test]
    fn add_v_i() {
        assert_eq!("VI", add("V", "I").unwrap());
    }

    #[test]
    fn add_i_v_understands_the_relative_order_of_v_and_i() {
        assert_eq!("VI", add("I", "V").unwrap());
    }

    #[test]
    fn add_i_iv_denormalizes_before_adding() {
        assert_eq!("V", add("I", "IV").unwrap());
    }

    #[test]
    fn add_l_i_supports_l() {
        assert_eq!("LI", add("L", "I").unwrap());
    }

    #[test]
    fn add_l_xi_understands_l_x_sort_order() {
        assert_eq!("LXI", add("L", "XI").unwrap());
    }

    #[test]
    fn add_fails_when_result_is_too_big_to_be_represented() {
        assert!(add("MCMXCIX", "MMCMXCIX").is_err());
    }
}
