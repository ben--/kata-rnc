use std::cmp::Ordering;

pub fn cmp(l: char, r: char) -> Ordering {
    if l == r {
        Ordering::Equal
    } else {
        match(l, r) {
            (_, 'M') => Ordering::Less,
            ('M', _) => Ordering::Greater,
            (_, 'D') => Ordering::Less,
            ('D', _) => Ordering::Greater,
            (_, 'C') => Ordering::Less,
            ('C', _) => Ordering::Greater,
            (_, 'L') => Ordering::Less,
            ('L', _) => Ordering::Greater,
            (_, 'X') => Ordering::Less,
            ('X', _) => Ordering::Greater,
            (_, 'V') => Ordering::Less,
            ('V', _) => Ordering::Greater,
            (_, _) => Ordering::Equal,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::cmp;
    use std::cmp::Ordering;

    #[test] fn i_equal_to_i() { assert_eq!(Ordering::Equal, cmp('I', 'I')); }

    #[test] fn i_less_than_v() { assert_eq!(Ordering::Less, cmp('I', 'V')); }
    #[test] fn v_greater_than_i() { assert_eq!(Ordering::Greater, cmp('V', 'I')); }
    #[test] fn v_equal_to_v() { assert_eq!(Ordering::Equal, cmp('V', 'V')); }

    #[test] fn x_greater_than_v() { assert_eq!(Ordering::Greater, cmp('X', 'V')); }
    #[test] fn v_less_than_x() { assert_eq!(Ordering::Less, cmp('V', 'X')); }
    #[test] fn x_equal_to_x() { assert_eq!(Ordering::Equal, cmp('X', 'X')); }

    #[test] fn l_greater_than_x() { assert_eq!(Ordering::Greater, cmp('L', 'X')); }
    #[test] fn x_less_than_l() { assert_eq!(Ordering::Less, cmp('X', 'L')); }
    #[test] fn l_equal_to_l() { assert_eq!(Ordering::Equal, cmp('L', 'L')); }

    #[test] fn c_greater_than_l() { assert_eq!(Ordering::Greater, cmp('C', 'L')); }
    #[test] fn l_less_than_c() { assert_eq!(Ordering::Less, cmp('L', 'C')); }
    #[test] fn c_equal_to_c() { assert_eq!(Ordering::Equal, cmp('C', 'C')); }

    #[test] fn d_greater_than_c() { assert_eq!(Ordering::Greater, cmp('D', 'C')); }
    #[test] fn c_less_than_d() { assert_eq!(Ordering::Less, cmp('C', 'D')); }
    #[test] fn d_equal_to_d() { assert_eq!(Ordering::Equal, cmp('D', 'D')); }

    #[test] fn m_greater_than_d() { assert_eq!(Ordering::Greater, cmp('M', 'D')); }
    #[test] fn d_less_than_m() { assert_eq!(Ordering::Less, cmp('D', 'M')); }
    #[test] fn m_equal_to_m() { assert_eq!(Ordering::Equal, cmp('M', 'M')); }
}
