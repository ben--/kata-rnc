pub fn denormalize(num: &str) -> String {
    let num = num.replace("IV", "IIII");
    let num = num.replace("IX", "VIIII");
    let num = num.replace("XL", "XXXX");
    num
}

#[cfg(test)]
mod tests {
    use super::denormalize;

    #[test]
    fn denormalize_iv() {
        assert_eq!("IIII", denormalize("IV"));
    }

    #[test]
    fn denormalize_ix() {
        assert_eq!("VIIII", denormalize("IX"));
    }

    #[test]
    fn denormalize_xiv_performs_a_partial_denormalization_on_tail() {
        assert_eq!("XIIII", denormalize("XIV"));
    }

    #[test]
    fn denormalize_xl() {
        assert_eq!("XXXX", denormalize("XL"));
    }
}
