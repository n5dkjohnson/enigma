mod enigma;
mod lib;
use crate::enigma::enigma::EnigmaWheel;
use crate::enigma::enigma::Enigma;

fn main() {
    let enigma_plugboard = EnigmaWheel::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_owned(), 0, 0);
    let mut enigma_wheel_i = EnigmaWheel::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ".to_owned(), 12, 0);
    let mut enigma_wheel_ii = EnigmaWheel::new("AJDKSIRUXBLHWTMCQGZNPYFVOE".to_owned(), 2, 0);
    let mut enigma_wheel_iii = EnigmaWheel::new("BDFHJLCPRTXVZNYEIWGAKMUSQO".to_owned(), 10, 0);
    let enigma_reflector = EnigmaWheel::new("YRUHQSLDPXNGOKMIEBFZCWVJAT".to_owned(), 0, 0);
    enigma_wheel_i.set_triggers(vec![17]);
    enigma_wheel_ii.set_triggers(vec![5]);
    enigma_wheel_iii.set_triggers(vec![22]);

    let message: String = "QMJIDO MZWZJFJR".to_owned();
    let mut enciphered: String = String::new();

    for chr in message.chars() {
        if chr > '@' && chr < '[' {
            let code: u16 = (chr as u16) - 64;
            let pos = &enigma_plugboard.right_to_left(code);
            if enigma_wheel_iii.rotate() {
                if enigma_wheel_ii.rotate() {
                    enigma_wheel_i.rotate();
                }
            }
            let pos = &enigma_wheel_iii.right_to_left(*pos);
            let pos = &enigma_wheel_ii.right_to_left(*pos);
            let pos = &enigma_wheel_i.right_to_left(*pos);
            let pos = &enigma_reflector.right_to_left(*pos);
            let pos = &enigma_wheel_i.left_to_right(*pos);
            let pos = &enigma_wheel_ii.left_to_right(*pos);
            let pos = &enigma_wheel_iii.left_to_right(*pos);

            enciphered.push(char::from_u32(*pos as u32 + 64).unwrap());
        } else {
            enciphered.push(chr);
        }
    }

    println!("{}", enciphered);
}
