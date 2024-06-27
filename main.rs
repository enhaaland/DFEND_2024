use std::io::{stdin,self,Write}; //read up how and why? //not sure how it exactly works
const SHIFT_VALUE: u8 = 4;

fn main() {
    let mut user_input = String::new(); 
    
    print!("Enter Plaintext: ");
    io::stdout().flush().unwrap(); //needed because stdout is line buffered - meaning shows up after newline.

    user_input = get_input(user_input);
    user_input = get_ciphertext(user_input);

    println!("Cipher Text: {}" , user_input);
}

fn get_input(mut input: String) -> String {
    stdin().read_line(&mut input).expect("Failed to readline"); 
    return input;
}

fn get_ciphertext(input: String) -> String {
    let mut cipher_text = String::new();
    let mut shifted_character;
    let mut redefined_character;
    for character in input.chars() { 
        redefined_character = character.to_ascii_lowercase() as u8; //u8 is common to make everything within the same realm
        match redefined_character {
            10 => break,
            32 => cipher_text.push(' '),
            _ => {
                shifted_character = (((redefined_character as u8) - 97 + SHIFT_VALUE) % 26 + 97) as char;
                cipher_text.push(shifted_character as char);
            }

        }
        
    }

    return cipher_text;
}
