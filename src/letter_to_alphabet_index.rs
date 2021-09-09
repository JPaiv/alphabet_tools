use std::char;

fn letter_alphabet_index(letter: str)-> i32 {
    let english_alphabet = (10..36).map(|i| char::from_digit(i, 36).unwrap()).collect::<Vec<_>>();
    let letter_index = english_alphabet.iter().position(|&r| r == 'a').unwrap();
    return letter_index 
}