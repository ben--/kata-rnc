pub fn normalize(num: &str) -> String {
    let num = num.replace("IIIII", "V");
    let num = num.replace("IIII", "IV");
    let num = num.replace("VV", "X");
    let num = num.replace("VIV", "IX");
    let num = num.replace("XXXXX", "L");
    let num = num.replace("XXXX", "XL");
    let num = num.replace("LL", "C");
    let num = num.replace("LXL", "XC");
    let num = num.replace("CCCCC", "D");
    let num = num.replace("CCCC", "CD");
    let num = num.replace("DD", "M");
    let num = num.replace("DCD", "CM");
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
    fn vviv_becomes_xiv() {
        assert_eq!("XIV", normalize("VVIV"));
    }

    #[test]
    fn xxxxx_becomes_l() {
        assert_eq!("L", normalize("XXXXX"));
    }

    #[test]
    fn xxxx_becomes_xl() {
        assert_eq!("XL", normalize("XXXX"));
    }

    #[test]
    fn lxxxx_becomes_xc() {
        assert_eq!("XC", normalize("LXXXX"));
    }

    #[test]
    fn lxl_becomes_xc() {
        assert_eq!("XC", normalize("LXL"));
    }

    #[test]
    fn lxxxxx_becomes_c() {
        assert_eq!("C", normalize("LXXXXX"));
    }

    #[test]
    fn ll_becomes_c() {
        assert_eq!("C", normalize("LL"));
    }

    #[test]
    fn llxl_becomes_cxl() {
        assert_eq!("CXL", normalize("LLXL"));
    }

    #[test]
    fn cccc_becomes_cd() {
        assert_eq!("CD", normalize("CCCC"));
    }

    #[test]
    fn ccccc_becomes_d() {
        assert_eq!("D", normalize("CCCCC"));
    }

    #[test]
    fn dcd_becomes_cm() {
        assert_eq!("CM", normalize("DCD"));
    }

    #[test]
    fn dd_becomes_m() {
        assert_eq!("M", normalize("DD"));
    }
}
