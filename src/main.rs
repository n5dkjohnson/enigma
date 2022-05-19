mod enigma;
mod lib;
use crate::enigma::enigma_machine::EnigmaMachine;

// TODO: Find some test material with plugboards, double-stepping and ring settings
// TODO: Make sure all comments are complete and accurate
// TODO: Add test cases for right-to-left and left-to-right operations to enigma crate

fn main() {
    let mut my_enigma = EnigmaMachine::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_owned(),
        "BDFHJLCPRTXVZNYEIWGAKMUSQO".to_owned(), 10, 0,
        "AJDKSIRUXBLHWTMCQGZNPYFVOE".to_owned(), 2, 0,
        "EKMFLGDQVZNTOWYHXUSPAIBRCJ".to_owned(), 12, 0,
        "YRUHQSLDPXNGOKMIEBFZCWVJAT".to_owned()
    );
    my_enigma.set_triggers(vec![22], vec![5], vec![17]);
    let transformed:String = my_enigma.transform_message("QMJIDO MZWZJFJR".to_owned());
    println!("{}", transformed);
}
