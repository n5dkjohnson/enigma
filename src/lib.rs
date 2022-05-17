pub mod lib {
    /* The Cipher trait provides methods for enciphering and deciphering the underlying object */
    pub trait Cipher {
        fn encipher(&self, message: &str) -> String;
        fn decipher(&self, message: &str) -> String;
    }
}