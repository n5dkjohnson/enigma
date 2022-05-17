mod enigma;
mod cipher;
mod lib;
use crate::enigma::enigma::EnigmaWheel;
use cipher::cipher::CipherWheel;
use cipher::cipher::Cipher;

fn main() {
    let wheel_i = EnigmaWheel::new("QWERTYUIOPASDFGHJKLZXCVBNM".to_owned(), 0, 0);
    let plugboard = CipherWheel::new("BADCFEHGJILKNMPORQTSVUXWZY".to_owned(), 0);

    println!{"{}", wheel_i.encipher("TESTING ENCIPHERING THIS MESSAGE")};
    println!{"{}", wheel_i.decipher("ZTLZOFU TFEOHITKOFU ZIOL DTLLQUT")};
    println!{"{}", plugboard.encipher("TESTING ENCIPHERING THIS MESSAGE")};
    println!{"{}", plugboard.decipher("SFTSJMH FMDJOGFQJMH SGJT NFTTBHF")};
}
