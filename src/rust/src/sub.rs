pub fn sub(num_l: &str, num_r: &str) -> Result<String, &'static str> {
    match num_l.find(num_r) {
        Some(begin) => {
            let end = begin + num_r.len();
            Ok(num_l[end..num_l.len()].to_string())
        },
        None => {
            Err("Romans don't know negative numbers")
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
}
