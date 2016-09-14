extern crate regex;

use self::regex::Regex;

pub fn valid(num: &str) -> bool {
    let re_text = concat!(
        "^",
        "(M(MM?)?)?",
        "(CM|CD|D|D?C(CC?)?)?",
        "(XC|XL|L|L?X(XX?)?)?",
        "(IX|IV|V|V?I(II?)?)?",
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

    #[test]
    fn ii_is_valid() {
        assert!(valid("II"));
    }

    #[test]
    fn iii_is_valid() {
        assert!(valid("III"));
    }

    #[test]
    fn iiii_is_not_valid() {
        assert!(!valid("IIII"));
    }

    #[test]
    fn iv_is_valid() {
        assert!(valid("IV"));
    }

    #[test]
    fn ivi_is_not_valid() {
        assert!(!valid("IVI"));
    }

    #[test]
    fn v_is_valid() {
        assert!(valid("V"));
    }

    #[test]
    fn vi_is_valid() {
        assert!(valid("VI"));
    }

    #[test]
    fn vii_is_valid() {
        assert!(valid("VII"));
    }

    #[test]
    fn viii_is_valid() {
        assert!(valid("VIII"));
    }

    #[test]
    fn ix_is_valid() {
        assert!(valid("IX"));
    }

    #[test]
    fn ixi_is_invalid() {
        assert!(!valid("IXI"));
    }

    #[test]
    fn ixv_is_invalid() {
        assert!(!valid("IXV"));
    }

    #[test]
    fn viv_is_invalid() {
        assert!(!valid("VIV"));
    }

    #[test]
    fn empty_is_invalid() {
        assert!(!valid(""));
    }

    #[test]
    fn x_is_valid() {
        assert!(valid("X"));
    }

    #[test]
    fn xi_is_valid() {
        assert!(valid("XI"));
    }

    #[test]
    fn xx_is_valid() {
        assert!(valid("XX"));
    }

    #[test]
    fn xxx_is_valid() {
        assert!(valid("XXX"));
    }

    #[test]
    fn xxxix_is_valid() {
        assert!(valid("XXXIX"));
    }

    #[test]
    fn xxxx_is_not_valid() {
        assert!(!valid("XXXX"));
    }

    #[test]
    fn xl_is_valid() {
        assert!(valid("XL"));
    }

    #[test]
    fn xlx_is_not_valid() {
        assert!(!valid("XLX"));
    }

    #[test]
    fn l_is_valid() {
        assert!(valid("L"));
    }

    #[test]
    fn lx_is_valid() {
        assert!(valid("LX"));
    }

    #[test]
    fn lxx_is_valid() {
        assert!(valid("LXX"));
    }

    #[test]
    fn lxxx_is_valid() {
        assert!(valid("LXXX"));
    }

    #[test]
    fn xc_is_valid() {
        assert!(valid("XC"));
    }

    #[test]
    fn xcx_is_invalid() {
        assert!(!valid("XCX"));
    }

    #[test]
    fn c_is_valid() {
        assert!(valid("C"));
    }

    #[test]
    fn cx_is_valid() {
        assert!(valid("CX"));
    }

    #[test]
    fn cc_is_valid() {
        assert!(valid("CC"));
    }

    #[test]
    fn ccc_is_valid() {
        assert!(valid("CCC"));
    }

    #[test]
    fn cccxc_is_valid() {
        assert!(valid("CCCXC"));
    }

    #[test]
    fn cccc_is_not_valid() {
        assert!(!valid("CCCC"));
    }

    #[test]
    fn cd_is_valid() {
        assert!(valid("CD"));
    }

    #[test]
    fn cdc_is_not_valid() {
        assert!(!valid("CDC"));
    }

    #[test]
    fn d_is_valid() {
        assert!(valid("D"));
    }

    #[test]
    fn dc_is_valid() {
        assert!(valid("DC"));
    }

    #[test]
    fn dcc_is_valid() {
        assert!(valid("DCC"));
    }

    #[test]
    fn dccc_is_valid() {
        assert!(valid("DCCC"));
    }

    #[test]
    fn cm_is_valid() {
        assert!(valid("CM"));
    }

    #[test]
    fn cmc_is_invalid() {
        assert!(!valid("CMC"));
    }

    #[test]
    fn m_is_valid() {
        assert!(valid("M"));
    }

    #[test]
    fn mc_is_valid() {
        assert!(valid("MC"));
    }

    #[test]
    fn mm_is_valid() {
        assert!(valid("MM"));
    }

    #[test]
    fn mmm_is_valid() {
        assert!(valid("MMM"));
    }

    #[test]
    fn mmmcm_is_valid() {
        assert!(valid("MMMCM"));
    }

    #[test]
    fn mmmm_is_invalid() {
        assert!(!valid("MMMM"));
    }
}
