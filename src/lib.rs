pub mod cipher {
    /* The Cipher trait provides methods for enciphering and deciphering the underlying object */
    pub trait Cipher {
        fn encipher(&self, message: &str) -> String;
        fn decipher(&self, message: &str) -> String;
    }

    /* A CipherWheel is a representation of a simple substitution cipher. It contains the following:
         A String called cipher which represents the encoded result of the alphabet
       A CipherWheel has the following functions available to it:
         new is a constructor that returns a new CipherWheel object given a cipher String as above
         encipher is a function that returns an enciphered String given a plaintext String using the encipherment provided in the 
            cipher variable
         decipher is a function that returns a plaintext String given an enciphered String using the encipherment provided in the
            cipher variable
       CipherWheel implements the trait Cipher
       Further work: To properly achieve the desired functionality of implementing an Enigma machine, the CipherWheel object needs to be extended to have an offset, a way to increment the offset, and a way to pass to subsequent wheels a signal to increment when necessary. To continue to function as a reflector and a plugboard, it also needs to ignore increment instruction and to pass on increment instructions. */
    pub struct CipherWheel {
        cipher: String
    }

    /* The new method for CipherWheel allows us to create a CipherWheel without exposing the cipher to users. After initial creation
       the CipherWheel object only allows encipher and decipher operations. NOTE: These methods probably allow deduction of the underlying cipher. If keeping the cipher private is desired, the encipher and decipher methods should not be exposed for
       unrestricted use. */
    impl CipherWheel {
        /* function: new
           input: String representing the enciphered alphabet
           output: CipherWheel object containing the specified cipher and encipher and decipher methods for its use
           limitations: The cipher cannot be changed once it is initially set */
        pub fn new(new_cipher: String) -> CipherWheel {
            CipherWheel{cipher: new_cipher}
        }
    }

    /* The implementation of the Cipher trait for a CipherWheel object */
    impl Cipher for CipherWheel {
        /* function: encipher
           input: String containing the Message to be enciphered
           output: String containing the enciphered message
           limitations: encipher only performs its functions on UPPERCASE STRINGS
           alogrithm: Each letter in the message is converted to an index representing its position in the alphabet,
            then the letter from cipher corresponding to that position is added to the encrypted String. Characters
            that are not uppercase Roman letters are not affected, and are retained in the output String unchanged. */
        fn encipher(&self, message: &str) -> String {
            let mut enciphered_text: String = String::new();

            for code in message.encode_utf16() {
                if code > 64 && code < 91 {
                    // take the ascii code for the letter in the plaintext, subtract 65 to optain a zero-based index
                    // into the alphabet, then return the letter corresponding to that index from the cipher
                    let encoded = self.cipher.chars().nth((code.checked_sub(65).unwrap()) as usize).unwrap();
                    enciphered_text.push(encoded);
                } else {
                    enciphered_text.push(char::from_u32(code as u32).unwrap());
                }
            }
            
            enciphered_text
        }

        /* function: decipher
           input: String containing the Message to be deciphered
           output: String containing the deciphered message
           limitations: decipher only performs its functions on UPPERCASE STRINGS
           alogrithm: Each letter in the message is lloked up in the cipher to determine its position, then the letter
           from the alphabet corresponding to that position is added to the decrypted String. Characters that are not 
           uppercase Roman letters are not affected, and are retained in the output String unchanged. */
        fn decipher(&self, message: &str) ->String {
            let mut plain_text: String = String::new();

            for chr in message.chars() {
                if chr > '@' && chr <'[' {
                    // find the character's position in the cipher key
                    let mut decoded = self.cipher.find(chr).unwrap() as u16;
                    // convert the position to the ASCII code for the corresponding letter of the alphabet
                    decoded = decoded.checked_add(65).unwrap();
                    plain_text.push(char::from_u32(decoded as u32).unwrap());
                } else {
                    plain_text.push(chr);
                }
            }
        
            plain_text
        }
    }

#[cfg(test)]
    #[test]
    // tests to see if basic substitution cipher enciphers the alphabet to the cipher key
    fn test_encipher() {
        let cipher = "DEFGHIJKLMNOPQRSTUVWXYZABC".to_owned();
        let caesar = CipherWheel::new(cipher.clone());
        let enciphered = caesar.encipher("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        assert_eq!(cipher, enciphered);
    }

    #[test]
    // tests to see if basic substition cipher deciphers the cipher key to the alphabet
    fn test_decipher() {
        let cipher = "DEFGHIJKLMNOPQRSTUVWXYZABC".to_owned();
        let caesar = CipherWheel::new(cipher.clone());
        let deciphered = caesar.decipher(&cipher);
        assert_eq!("ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_owned(), deciphered);
    }
}