extern crate regex;

use self::regex::Regex;

pub fn valid(num: &str) -> bool {
    let re = Regex::new("I").unwrap();
    re.is_match(num)
}

#[cfg(test)]
mod tests {
    use super::valid;

    #[test]
    fn i_is_valid() {
        assert!(valid("I"));
    }

    #[test]
    fn lower_i_is_invalid() {
        assert!(!valid("i"));
    }
}
