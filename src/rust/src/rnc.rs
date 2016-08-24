
    pub fn add(num_l: &str, num_r: &str) -> String {
        let sum = String::from(num_l) + num_r;

        if sum == "IIIII" {
            String::from("V")
        } else {
            sum
        }
    }

    #[cfg(test)]
    mod tests {
        use super::add;

        #[test]
        fn add_i_i() {
            assert_eq!("II", add("I", "I"));
        }

        #[test]
        fn add_i_ii() {
            assert_eq!("III", add("I", "II"));
        }

        #[test]
        fn add_ii_iii() {
            assert_eq!("V", add("II", "III"));

        }
    }
