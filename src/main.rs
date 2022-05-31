mod enigma;
mod lib;
use crate::enigma::enigma_machine::EnigmaMachine;

// TODO: Find some test material with plugboards, double-stepping and ring settings
// TODO: Make sure all comments are complete and accurate

fn main() {
    let mut my_enigma = EnigmaMachine::new(
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_owned(),           // plugboard cipher
        "BDFHJLCPRTXVZNYEIWGAKMUSQO".to_owned(), 10, 2,    // rightmost rotor cipher, offset and ring setting
        "AJDKSIRUXBLHWTMCQGZNPYFVOE".to_owned(), 2, 9,     // middle rotor cipher, offset and ring setting
        "EKMFLGDQVZNTOWYHXUSPAIBRCJ".to_owned(), 12, 7,    // leftmost rotor cipher, offset and ring setting
        "YRUHQSLDPXNGOKMIEBFZCWVJAT".to_owned()            // reflector cipher
    );
    my_enigma.set_triggers(vec![22], vec![5], vec![17]);   // set rotor tunrover points
    let transformed:String = my_enigma.transform_message("ENIGMA REVEALED".to_owned());
    println!("{}", transformed);
    my_enigma.set_rotor_positions(10, 2, 12);
    let retransformed:String = my_enigma.transform_message(transformed);
    println!("{}", retransformed);
    my_enigma.set_rotor_positions(10, 2, 12);
    let encoded:String = my_enigma.transform_message("THE TIME HAS COME THE WALRUS SAID TO TALK OF MANY THINGS OF SHOES AND SHIPS AND SEALING-WAX OF CABBAGES AND KINGS AND WHY THE SEA IS BOILING HOT AND WHETHER PIGS HAVE WINGS".to_owned());
    println!("{}", encoded);
    my_enigma.set_rotor_positions(10, 2, 12);
    let decoded:String = my_enigma.transform_message(encoded);
    println!("{}", decoded);
}
