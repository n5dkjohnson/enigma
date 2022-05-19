pub mod enigma_wheel {
    use enigma::lib::Cipher;

    // The Enigma Trait provides methods for rotating the offset, and propogating rotating the offset 
    pub trait Enigma {
        fn rotate(&mut self) -> bool;
        fn set_rotor_position(&mut self, rotor_position: u16);
        fn set_triggers(&mut self, triggers: Vec<u16>);
        fn right_to_left(&self, position: u16) -> u16;
        fn left_to_right(&self, position: u16) -> u16;
    }

    /* An EnigmaWheel is a representation of a rotating offset substitution cipher. It contains the following:
         A String called cipher which represents the encoded result of the alphabet
         A u16 called rotor_position that represent the number of characters an input char is shifted prior to enciphering
          Shifts move baclwards in the alphabet (e.g.: C shifts by 2 to A), and are stored mod 26
         A u16 called ring_setting that represents the number of characters an output char is shifter after enciphering
          Shifts move forward in the aplhabet (e.g.: A shifts by 2 to A), and are stored mod 26
       An EnigmaWheel has the following functions available to it:
         new is a constructor that returns a new EnigmaWheel object given a cipher String, offset u16 and setting u16 as above
         encipher is a function that returns an enciphered String given a plaintext String using the encipherment provided in the 
            cipher variable
         decipher is a function that returns a plaintext String given an enciphered String using the encipherment provided in the
            cipher variable
         rotate is a function that increments the offset by one mod 26. This rotation is propagated to successive wheels if necessary
         set_rotor_position is a function the sets or resets the initial position of the rotor
         set_triggers is a function that sets the turnover points of the rotor
         right_to_left takes the position of an input signal on the right side of the rotor and returns the position of the output
            signal on the left hand side of the rotor
         left_to_right takes the position of an input signal on the left side of the rotor and returns the position of the output 
            signal on the right hand side of the rotor
       EnigmaWheel implements the traits Cipher and Enigma */
    pub struct EnigmaWheel {
        cipher: String,
        rotor_position: u16,
        ring_setting: u16,
        triggers: Vec<u16>
    }

    /* The new method for EnigmaWheel allows us to create a EnigmaWheel without exposing the cipher to users. After initial creation
       the EnigmaWheel object only allows encipher and decipher operations. NOTE: These methods probably allow deduction of the underlying cipher. If keeping the cipher private is desired, the encipher and decipher methods should not be exposed for
       unrestricted use. */
    impl EnigmaWheel {
        /* function: new
           inputs: String representing the enciphered alphabet
                   u16 representing the shift applied to the original letter before ciphering
                   u16 representing the shift applied to the enciphered letter
           output: EnigmaWheel object containing the specified cipher and encipher and decipher methods for its use
           limitations: The cipher cannot be changed once it is initially set */
        pub fn new(new_cipher: String, new_offset: u16, new_setting: u16) -> EnigmaWheel {
            EnigmaWheel{
                cipher: new_cipher, 
                rotor_position: new_offset.checked_rem(26).unwrap(), 
                ring_setting: new_setting.checked_rem(26).unwrap(),
                triggers: vec![]
            }
        }
    }

    /* The implementation of the Cipher trait for a EnigmaWheel object */
    impl Cipher for EnigmaWheel {
        /* function: encipher
           input: String containing the Message to be enciphered
           output: String containing the enciphered message
           limitations: encipher only performs its functions on UPPERCASE STRINGS
           alogrithm: Each letter in the message is converted to an index representing its position in the alphabet, shited by the offset specified in the cipher, then the letter from cipher corresponding to that position is added to the encrypted String. Characters that are not uppercase Roman letters are not affected, and are retained in the output String unchanged. */
        fn encipher(&self, message: &str) -> String {
            let mut enciphered_text: String = String::new();

            for mut code in message.encode_utf16() {
                if code > 64 && code < 91 {
                    // take the ascii code for the letter in the plaintext, subtract 65 to obtain a zero-based index
                    code = code.checked_sub(65).unwrap();
                    // then add the offset to the index mod 26 to obtain shifted index of source character
                    code = code.checked_add(26-self.rotor_position).unwrap().checked_rem(26).unwrap();
                    // return the letter corresponding to that index from the cipher
                    let mut enciphered_code = self.cipher.chars().nth((code) as usize).unwrap() as u16;
                    // shift the letter obtained by the ring setting, adjusting to keep in range
                    enciphered_code += self.ring_setting;
                    if enciphered_code > 90 {
                        enciphered_code -= 26;
                    }
                    let encoded = char::from_u32(enciphered_code as u32).unwrap();
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
                    // undo the shift caused by the ring setting adjusting to keep character in range
                    let mut code = chr as u32 - self.ring_setting as u32;
                    if code < 65 {
                        code += 26;
                    }
                    // find the character's position in the cipher key
                    let mut decoded = self.cipher.find(char::from_u32(code).unwrap()).unwrap() as u16;
                    // modify position based on current rotor_position
                    decoded = decoded.checked_add(self.rotor_position).unwrap().checked_rem(26).unwrap();
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

    // The implementation of the Enigma Trait for an EnigmaWheel object
    impl Enigma for EnigmaWheel {
        /* function: rotate
           input: none
           output: bool indicating whether the next wheel in the sequence should be rotated as well
           limitations: none obvious at this time
           algorithm: increments the rotor position mod 26, then checks to see if any of the triggers for rotating the next wheel were hit */
        fn rotate(&mut self) -> bool {
            let mut result = false;
            self.rotor_position += 1;
            self.rotor_position = self.rotor_position.checked_rem(26).unwrap();
            for i in &self.triggers[..] {
                if i == &self.rotor_position {
                    result = true;
                }
            }

            result
        }

        /* function: set_rotor_position
           input: u16 representing the current rotor position mod 26
           output: none
           limitations: none obvious at this time
           algorithm: sets the specified rotor position mod 26 */
        fn set_rotor_position(&mut self, rotor_position: u16) {
                self.rotor_position = rotor_position.checked_rem(26).unwrap();
        }

        /* function: set_triggers
           input: Vector of u16s representing the positions which trigger the next wheel to rotate
           output: none
           limitations: none obvious at this time
           algorithm: sets the list of trigger points to the supplied list of trigger points */
        fn set_triggers(&mut self, triggers: Vec<u16>) {
                self.triggers = triggers;
        }

        /* function: right_to_left
           input: u16 representing the index of the input
           output: u16 representing the index of the output
           limitations: none obvious at this time
           algorithm: traces the input through the wheel wiring to the output accounting for start position */
        fn right_to_left(&self, position: u16) -> u16 {
            let index: u16 = (position + self.rotor_position - 1).checked_rem(26).unwrap();
            // println!("{0} {1} {2}", position, self.rotor_position, position + self.rotor_position -1);
            let chr: char = self.cipher.chars().nth(index as usize).unwrap();
            // print!("{}", chr);
            (26 - self.rotor_position + (chr as u16 - 64)).checked_rem(26).unwrap()
        }

        /* function: right_to_left
           input: u16 representing the index of the input
           output: u16 representing the index of the output
           limitations: none obvious at this time
           algorithm: traces the input through the wheel wiring to the output accounting for start position */
        fn left_to_right(&self, position: u16) -> u16 {
            let index: u16 = (position + self.rotor_position).checked_rem(26).unwrap();
            // println!("{0} {1} {2}", position, self.rotor_position, index);
            let decoded = self.cipher.find(char::from_u32(index as u32 + 64).unwrap()).unwrap() as u16;
            // print!("{}", char::from_u32(index as u32 + 64).unwrap());
            (26 - self.rotor_position + (decoded + 1)).checked_rem(26).unwrap()
        }
    }

    #[cfg(test)]
    #[test]
    // tests to see if basic substitution cipher enciphers the alphabet to the cipher key
    fn test_encipher() {
        let cipher = "DEFGHIJKLMNOPQRSTUVWXYZABC".to_owned();
        let enigma = EnigmaWheel::new(cipher.clone(), 0, 0);
        let enciphered = enigma.encipher("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        assert_eq!(cipher, enciphered);
    }

    #[test]
    // tests to see if basic substition cipher deciphers the cipher key to the alphabet
    fn test_decipher() {
        let cipher = "DEFGHIJKLMNOPQRSTUVWXYZABC".to_owned();
        let enigma = EnigmaWheel::new(cipher.clone(), 0, 0);
        let deciphered = enigma.decipher(&cipher);
        assert_eq!("ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_owned(), deciphered);
    }

    #[test]
    // tests to see if basic substitution cipher with offset enciphers the alphabet to the cipher key
    fn test_encipher_with_shift() {
        let cipher = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_owned();
        let enigma = EnigmaWheel::new(cipher.clone(), 23, 0);
        let enciphered = enigma.encipher("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        assert_eq!("DEFGHIJKLMNOPQRSTUVWXYZABC".to_owned(), enciphered);
    }

    #[test]
    // tests to see if basic substition cipher with offset deciphers the cipher key to the alphabet
    fn test_decipher_with_shift() {
        let cipher = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_owned();
        let enigma = EnigmaWheel::new(cipher.clone(), 23, 0);
        let deciphered = enigma.decipher("DEFGHIJKLMNOPQRSTUVWXYZABC");
        assert_eq!("ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_owned(), deciphered);
    }

    #[test]
    // Tests to see if basic substitution cipher with rotating offset enciphers the alphabet to a repeating character string
    fn test_encipher_with_rotating_shift() {
        let cipher = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_owned();
        let mut enigma = EnigmaWheel::new(cipher.clone(), 23, 0);
        let mut enciphered: String = String::new();
        for i in "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
            let i_str = &format!("{}", i);
            enigma.rotate();
            enciphered.push(enigma.encipher(i_str).chars().next().unwrap());
        };
        assert_eq!("CCCCCCCCCCCCCCCCCCCCCCCCCC".to_owned(), enciphered);
    }

    #[test]
    // Tests to see if basic substitution cipher with rotating offset deciphers a repeating character string to the alphabet
    fn test_decipher_with_rotating_shift() {
        let cipher = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_owned();
        let mut enigma = EnigmaWheel::new(cipher.clone(), 23, 0);
        let mut enciphered: String = String::new();
        for i in "CCCCCCCCCCCCCCCCCCCCCCCCCC".chars() {
            let i_str = &format!("{}", i);
            enigma.rotate();
            enciphered.push(enigma.decipher(i_str).chars().next().unwrap());
        };
        assert_eq!("ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_owned(), enciphered);
    }

    #[test]
    // Tests to see if basic substitution cipher with offset and ring setting enciphers the alphabet to a known character string
    fn test_encipher_with_shift_and_ring_setting() {
        let cipher = "EKMFLGDQVZNTOWYHXUSPAIBRCJ".to_owned();
        let mut enigma = EnigmaWheel::new(cipher.clone(), 0, 1);
        enigma.rotate();
        let enciphered = enigma.encipher("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        assert_eq!("KFLNGMHERWAOUPXZIYVTQBJCSD".to_owned(), enciphered);
    }

    #[test]
    // Tests to see if basic substitution cipher with offset and ring setting deciphers a known character string to the alphabet
    fn test_decipher_with_shift_and_ring_setting() {
        let cipher = "EKMFLGDQVZNTOWYHXUSPAIBRCJ".to_owned();
        let mut enigma = EnigmaWheel::new(cipher.clone(), 0, 1);
        enigma.rotate();
        let enciphered = enigma.decipher("KFLNGMHERWAOUPXZIYVTQBJCSD");
        assert_eq!("ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_owned(), enciphered);
    }

    #[test]
    // Tests to see if basic substitution cipher with rotating offset and ring setting enciphers the alphabet to a repeating character string
    fn test_encipher_with_rotating_shift_and_ring_setting() {
        let cipher = "EKMFLGDQVZNTOWYHXUSPAIBRCJ".to_owned();
        let mut enigma = EnigmaWheel::new(cipher.clone(), 0, 2);
        let mut enciphered: String = String::new();
        for i in "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
            let i_str = &format!("{}", i);
            enigma.rotate();
            enciphered.push(enigma.encipher(i_str).chars().next().unwrap());
        };
        assert_eq!("LLLLLLLLLLLLLLLLLLLLLLLLLL".to_owned(), enciphered);
    }

    #[test]
    // Tests to see if basic substitution cipher with rotating offset and ring settting deciphers a repeating character string to the alphabet
    fn test_decipher_with_rotating_shift_and_ring_setting() {
        let cipher = "EKMFLGDQVZNTOWYHXUSPAIBRCJ".to_owned();
        let mut enigma = EnigmaWheel::new(cipher.clone(), 0, 2);
        let mut enciphered: String = String::new();
        for i in "LLLLLLLLLLLLLLLLLLLLLLLLLL".chars() {
            let i_str = &format!("{}", i);
            enigma.rotate();
            enciphered.push(enigma.decipher(i_str).chars().next().unwrap());
        };
        assert_eq!("ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_owned(), enciphered);
    }

    #[test]
    // Tests to see if the EnigmaWheels rotation properly triggers the rotation of the subsequent wheel
    fn test_rotate_propagation() {
        let cipher = "EKMFLGDQVZNTOWYHXUSPAIBRCJ".to_owned();
        let mut enigma = EnigmaWheel::new(cipher.clone(), 0, 2);
        enigma.set_triggers(vec![6, 13, 20]);
        let mut i: u16 = 0;
        while i<26 {
            let prop: bool = enigma.rotate();
            i += 1;
            assert_eq!(i.checked_rem(7).unwrap() == 6, prop);
        }
 
    }

    #[test]
    // Tests to see if the EnigmaWheel properly transforms position in the right-to-left direction
    fn test_right_to_left() {
        let wheel = EnigmaWheel::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ".to_owned(), 0, 0);
        let new_pos = wheel.right_to_left(12);
        assert_eq!(20, new_pos);
    }

    #[test]
    // Tests to see if the EnigmaWheel properly transforms position in the left-to-right direction
    fn test_lett_to_right() {
        let wheel = EnigmaWheel::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ".to_owned(), 0, 0);
        let new_pos = wheel.left_to_right(20);
        assert_eq!(12, new_pos);
    }
}

pub mod enigma_machine {
    use crate::enigma::enigma_wheel::EnigmaWheel;
    use crate::enigma::enigma_wheel::Enigma;

    /* An EnigmaStructure is a representation of a complete Enigma machine. It contains the following:
         An EnigmaWheel representing the plugboard
         An EnigmaWheel representing the rightmost Enigma wheel
         An EnigmaWheel representing the middle Enigma wheel
         An EnigmaWheel representing the leftmost Enigma wheel
         An EnigmaWheel representing the reflector
       An EnigmaWheel has the following functions available to it:
         new is a constructor that returns a new EnigmaMachine object given the components' ciphers String, offsets u16 and settings u16 as above
         set_triggers is a function that sets the rotating trigger points of the three wheels given their triggers Vec<u16>
         transform is a function that returns a plaintext String given an enciphered String or an enciphered String given a plaintext String using the setting provided for the EnigmaMachine */
    pub struct EnigmaMachine {
        plugboard: EnigmaWheel,
        right_wheel: EnigmaWheel,
        middle_wheel: EnigmaWheel,
        left_wheel: EnigmaWheel,
        reflector: EnigmaWheel
    }

    impl EnigmaMachine {
        pub fn new(pb_cipher: String, 
               rw_cipher: String, rw_offset: u16, rw_setting: u16,
               mw_cipher: String, mw_offset: u16, mw_setting: u16,
               lw_cipher: String, lw_offset: u16, lw_setting: u16,
               rf_cipher: String
            ) -> EnigmaMachine {
                EnigmaMachine {
                    plugboard: EnigmaWheel::new(pb_cipher, 0, 0),
                    right_wheel: EnigmaWheel::new(rw_cipher, rw_offset, rw_setting),
                    middle_wheel: EnigmaWheel::new(mw_cipher, mw_offset, mw_setting),
                    left_wheel: EnigmaWheel::new(lw_cipher, lw_offset, lw_setting),
                    reflector: EnigmaWheel::new(rf_cipher, 0, 0)
                }
            }
        
        pub fn set_triggers(&mut self, rw_triggers: Vec<u16>, mw_triggers: Vec<u16>, lw_triggers: Vec<u16>) {
            self.right_wheel.set_triggers(rw_triggers);
            self.middle_wheel.set_triggers(mw_triggers);
            self.left_wheel.set_triggers(lw_triggers);
        }

        pub fn transform_message(&mut self, message: String) -> String{
            let mut enciphered: String = String::new();

            for chr in message.chars() {
                if chr > '@' && chr < '[' {
                    let code: u16 = (chr as u16) - 64;
                    let pos = &self.plugboard.right_to_left(code);
                    if self.right_wheel.rotate() {
                        if self.middle_wheel.rotate() {
                            self.left_wheel.rotate();
                        }
                    }
                    let pos = &self.right_wheel.right_to_left(*pos);
                    let pos = &self.middle_wheel.right_to_left(*pos);
                    let pos = &self.left_wheel.right_to_left(*pos);
                    let pos = &self.reflector.right_to_left(*pos);
                    let pos = &self.left_wheel.left_to_right(*pos);
                    let pos = &self.middle_wheel.left_to_right(*pos);
                    let pos = &self.right_wheel.left_to_right(*pos);
        
                    enciphered.push(char::from_u32(*pos as u32 + 64).unwrap());
                } else {
                    enciphered.push(chr);
                }
            }
        
            enciphered
        }
    }

    #[test]
    // This deciphers a known message with known machine settings to ensure the EnigmaMachine is working properlsy
    fn test_full_machine() {
        let mut my_enigma = EnigmaMachine::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_owned(),
        "BDFHJLCPRTXVZNYEIWGAKMUSQO".to_owned(), 10, 0,
        "AJDKSIRUXBLHWTMCQGZNPYFVOE".to_owned(), 2, 0,
        "EKMFLGDQVZNTOWYHXUSPAIBRCJ".to_owned(), 12, 0,
        "YRUHQSLDPXNGOKMIEBFZCWVJAT".to_owned()
    );
    my_enigma.set_triggers(vec![22], vec![5], vec![17]);
    let transformed:String = my_enigma.transform_message("QMJIDO MZWZJFJR".to_owned());
    assert_eq!("ENIGMA REVEALED", transformed);
    }
}
