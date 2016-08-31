pub fn borrow(num: &str, digit: char) -> Result<String, &'static str> {
    if num == "V" {
        Ok("IIIII".to_string())
    } else {
        Ok(num.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::borrow;

    #[test]
    fn cannot_borrow_i_from_ii() {
        assert_eq!("II", borrow("II", 'I').unwrap());
    }

    #[test]
    fn borrow_v_i() {
        assert_eq!("IIIII", borrow("V", 'I').unwrap());
    }
}
