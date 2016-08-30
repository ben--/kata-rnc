pub fn sub(num_l: &str, num_r: &str) -> String {
    let begin = num_l.find(num_r).unwrap();
    let end = begin + num_r.len();
    num_l[end..num_l.len()].to_string()
}

#[cfg(test)]
mod tests {
    use super::sub;

    #[test]
    fn sub_ii_i() {
        assert_eq!("I", sub("II", "I"));
    }

    #[test]
    fn sub_iii_i() {
        assert_eq!("II", sub("III", "I"));
    }
}
