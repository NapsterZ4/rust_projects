use std::*;
use std::convert::TryInto;

const GENERATOR: i32 = 223;
const MODULUS: i32 = 36389;

fn h_fn<'a>(first_half: &'a [u8], second_half: &'a [u8]) -> Vec<u8> {
    let mod_exp: i32 = 10_i32.pow(first_half.len().try_into().unwrap()) % MODULUS;
    let mut hard_code_bit: u8 = 0;

    for i in 0..first_half.len() {
        hard_code_bit = hard_code_bit ^ ((first_half[i]) & (second_half[i])) % 2;
    }

    let exp = mod_exp.to_be_bytes();
    let hard = hard_code_bit.to_be_bytes();
    let c_vec = concatenate_arrays(&exp, &second_half);
    let final_vec = concatenate_arrays(&c_vec, &hard);

    final_vec
}

fn concatenate_arrays<'a, T: Clone>(x: &'a [T], y: &'a [T]) -> Vec<T> {
    let mut concat = x.to_vec();
    concat.extend_from_slice(y);

    concat
}

fn g_fn(initial_seed: &[u8], length: i32) -> Vec<u8> {
    let mut binary_string: Vec<u8> = initial_seed.to_vec();
    let mut result: Vec<u8> = [].to_vec();

    for _i in 0..length {
        let first_half: &[u8] = &binary_string[..(binary_string.len()) / 2];
        let second_half: &[u8] = &binary_string[(binary_string.len()) / 2..];
        binary_string = h_fn(first_half, second_half);
        result = concatenate_arrays(&result, &binary_string[binary_string.len() - 1..binary_string.len()]);
        binary_string = binary_string[..binary_string.len() - 1].to_vec(); // Tomar el ultimo
        // elemento y asignar nuevamente al binary_string


        println!("BINARY STRING: {:?}", binary_string);
        println!("RESULT: {:?}", result);
    }

    result
}

fn xor(message: &[u8], key: &[u8]) -> Vec<u8> {
    let mut xor: Vec<u8> = Vec::new();

    for n in 0..message.len() {
        xor.push(message[n] ^ key[n]);
    }

    xor.to_vec()
}

fn convert_string(message: Vec<u8>) -> String {
    String::from_utf8(message).unwrap()
}

fn controller() {
    let message: &[u8] = "Encrypt, this message has testing for data science Engineering".as_bytes();
    let key: &[u8] = "Einsten is the best".as_bytes();
    let length_message: usize = message.len();
    let length_key: usize = key.len();

    if length_key <= length_message {
        let result_password: Vec<u8> = g_fn(key, length_message as i32);
        let string_result: String = convert_string(result_password);
        let password_to: &[u8] = string_result.as_bytes();

        let encrypt_message: Vec<u8> = xor(message, password_to);
        let encrypt_message_bytes: String = convert_string(encrypt_message);

        println!("The Result length: {:?}", string_result.len());
        println!("String text: {:?}", encrypt_message_bytes);
    } else {
        println!("Insert a key smaller than the input text for encrypt");
    }
}

fn main() {
    controller();
}