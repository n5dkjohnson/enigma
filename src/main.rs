use enigma::cipher::CipherWheel;
use enigma::cipher::Cipher;

fn main() {
    let wheel_i = CipherWheel::new("QWERTYUIOPASDFGHJKLZXCVBNM".to_owned());

    println!{"{}", wheel_i.encipher("TESTING ENCIPHERING THIS MESSAGE")};
    println!{"{}", wheel_i.decipher("ZTLZOFU TFEOHITKOFU ZIOL DTLLQUT")};
}
