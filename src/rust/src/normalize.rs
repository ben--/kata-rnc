pub fn normalize(num: &str) -> String {
    let num = num.replace("IIIII", "V");
    let num = num.replace("IIII", "IV");
    let num = num.replace("VIV", "IX");
    let num = num.replace("VV", "X");
    let num = num.replace("XXXXX", "L");
    num
}

#[cfg(test)]
mod tests {
    use super::normalize;

    #[test]
    fn iiiii_becomes_v() {
        assert_eq!("V", normalize("IIIII"));
    }

    #[test]
    fn iiii_becomes_iv() {
        assert_eq!("IV", normalize("IIII"));
    }

    #[test]
    fn vv_becomes_x() {
        assert_eq!("X", normalize("VV"));
    }

    #[test]
    fn viiii_becomes_ix() {
        assert_eq!("IX", normalize("VIIII"));
    }

    #[test]
    fn viiiii_becomes_x() {
        assert_eq!("X", normalize("VIIIII"));
    }

    #[test]
    fn xxxxx_becomes_l() {
        assert_eq!("L", normalize("XXXXX"));
    }
}
