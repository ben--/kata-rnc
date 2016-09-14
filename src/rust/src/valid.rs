extern crate regex;

use self::regex::Regex;

pub fn valid(num: &str) -> bool {
    let re_text = concat!(
        "^",
        "I",
        "$");

    let re = Regex::new(re_text).unwrap();
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

    #[test]
    fn prefixes_are_not_valid() {
        assert!(!valid(" I"));
    }

    #[test]
    fn suffixes_are_not_valid() {
        assert!(!valid("I "));
    }
}
