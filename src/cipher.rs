pub mod cipher {
    pub trait Cipher {
        fn encipher(&self, message: &str) -> String;
        fn decipher(&self, message: &str) -> String;
    }

    pub struct CipherWheel {
        cipher: String
    }

    impl Cipher for CipherWheel {
        fn encipher(&self, message: &str) -> String {
            let mut enciphered_text: String = String::new();

            for code in message.encode_utf16() {
                if code > 64 && code < 91 {
                    let encoded = self.cipher.chars().nth((code.checked_sub(65).unwrap()) as usize).unwrap();
                    enciphered_text.push(encoded);
                } else {
                    enciphered_text.push(char::from_u32(code as u32).unwrap());
                }
            }
            
            enciphered_text
        }

        fn decipher(&self, message: &str) ->String {
            let mut plain_text: String = String::new();

            for chr in message.chars() {
                if chr > '@' && chr <'[' {
                    let mut decoded = self.cipher.find(chr).unwrap() as u16;
                    decoded = decoded.checked_add(65).unwrap();
                    plain_text.push(char::from_u32(decoded as u32).unwrap());
                } else {
                    plain_text.push(chr);
                }
            }
        
            plain_text
        }
    }
}