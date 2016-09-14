pub fn valid(num: &str) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::valid;

    #[test]
    fn i_is_valid() {
        assert!(valid("I"));
    }
}
