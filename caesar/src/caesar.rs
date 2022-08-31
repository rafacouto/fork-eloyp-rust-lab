pub struct Caesar {
    rotated_encrypt: Vec<char>,
    rotated_decrypt: Vec<char>,
}

impl Caesar {
    pub fn with_key(key: i32) -> Self {
        Self {
            rotated_encrypt: Caesar::precalc_rotated(key),
            rotated_decrypt: Caesar::precalc_rotated(-key),
        }
    }

    const ASCII_A_LOWER: u8 = b'a';
    const ASCII_A_UPPER: u8 = b'A';

    #[inline]
    pub fn encrypt(&self, s: &str) -> String {
        Caesar::rotate_str(s, &self.rotated_encrypt)
    }

    #[inline]
    pub fn decrypt(&self, s: &str) -> String {
        Caesar::rotate_str(s, &self.rotated_decrypt)
    }

    fn rotate_str(s: &str, rotated_chars: &[char]) -> String {
        s.chars()
            .map(|c| match c.is_ascii_alphabetic() {
                false => c,
                true => match c.is_ascii_uppercase() {
                    true => rotated_chars[(c as u8 - Caesar::ASCII_A_UPPER) as usize],
                    false => rotated_chars[(c as u8 - Caesar::ASCII_A_LOWER) as usize]
                        .to_ascii_lowercase(),
                },
            })
            .collect()
    }

    fn precalc_rotated(key: i32) -> Vec<char> {
        let mut offset = key % 26;
        if offset.is_negative() {
            offset += 26;
        }
        (offset as u8..26)
            .chain(0..offset as u8)
            .map(|ascii| (Caesar::ASCII_A_UPPER + ascii) as char)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_encrypts_basic_string() {
        let caesar = Caesar::with_key(1);
        let result = caesar.encrypt("ABC");
        assert_eq!("BCD", result);
    }

    #[test]
    fn it_decrypts_basic_string() {
        let caesar = Caesar::with_key(1);
        let result = caesar.decrypt("BCD");
        assert_eq!("ABC", result);
    }

    #[test]
    fn it_ignores_but_keeps_non_alphabet_characters() {
        let caesar = Caesar::with_key(1);
        let result = caesar.encrypt("(ABC)D");
        assert_eq!("(BCD)E", result);
    }

    #[test]
    fn it_respects_spaces() {
        let caesar = Caesar::with_key(1);
        let result = caesar.encrypt("A B C");
        assert_eq!("B C D", result);
    }

    #[test]
    fn it_respects_multiline() {
        let caesar = Caesar::with_key(1);
        let result = caesar.encrypt("A \n B \n C");
        assert_eq!("B \n C \n D", result);
    }

    #[test]
    fn it_respects_capitalization() {
        let caesar = Caesar::with_key(1);
        let result = caesar.encrypt("ABC");
        assert_eq!("BCD", result);
        let result = caesar.encrypt("abc");
        assert_eq!("bcd", result);
    }

    #[test]
    fn it_ignores_but_keeps_utf8_chars() {
        let caesar = Caesar::with_key(1);
        let result = caesar.encrypt("ЗaЗ");
        assert_eq!("ЗbЗ", result)
    }

    #[test]
    fn it_handles_last_alpha_pos_encrypt() {
        let caesar = Caesar::with_key(26);
        let result = caesar.encrypt("ABC");
        assert_eq!("ABC", result);
    }

    #[test]
    fn it_handles_last_alpha_pos_decrypt() {
        let caesar = Caesar::with_key(26);
        let result = caesar.decrypt("ABC");
        assert_eq!("ABC", result);
    }

    #[test]
    // Letters close to the end and displacement exceeds last alpha.
    fn it_handles_relative_upper_overflow() {
        let caesar = Caesar::with_key(3);
        let result = caesar.encrypt("XY");
        assert_eq!("AB", result);
    }

    #[test]
    // Letters close to the end and displacement exceeds last alpha.
    fn it_handles_relative_lower_overflow() {
        let caesar = Caesar::with_key(3);
        let result = caesar.decrypt("BC");
        assert_eq!("YZ", result);
    }

    #[test]
    // Two times the alphabet + 2 (forward).
    fn it_handles_upper_bound_overflow() {
        let caesar = Caesar::with_key(54);
        let result = caesar.encrypt("ABC");
        assert_eq!("CDE", result);
    }

    #[test]
    // Two times the alphabet + 3 (backward).
    fn it_handles_lower_bound_overflow() {
        let caesar = Caesar::with_key(55);
        let result = caesar.decrypt("ABC");
        assert_eq!("XYZ", result);
    }

    #[test]
    fn it_returns_same_on_no_key() {
        let caesar = Caesar::with_key(0);
        let result = caesar.encrypt("ABC");
        assert_eq!("ABC", result);
    }

    #[test]
    fn wikipedia_test() {
        let plaintext = "THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG";
        let ciphertext = "QEB NRFZH YOLTK CLU GRJMP LSBO QEB IXWV ALD";
        let caesar = Caesar::with_key(-3);
        let result = caesar.encrypt(plaintext);
        assert_eq!(ciphertext, result);
        let result = caesar.decrypt(ciphertext);
        assert_eq!(plaintext, result);
    }
}
