use std::*;
use std::convert::TryInto;

const SEED_SIZE: i32 = 16;
const GENERATOR: usize = 223;
const MODULUS: i32 = 36389;
const SEED: &[u8] = "1001100001110001".as_bytes();


fn h_fn<'a>(first_half: &'a[u8], second_half: &'a[u8]) -> &'a[u8] {
    let mut mod_exp: i32 = 2_i32.pow(first_half.len().try_into().unwrap()) % MODULUS;
    let mut hard_code_bit: u8 = 0;

    for i in 0..first_half.len() {
        hard_code_bit = hard_code_bit ^ ((first_half[i]) & (second_half[i])) % 2;
    }

    mod_exp + second_half + hard_code_bit
}

fn g_fn(initial_seed: &[u8]) -> &[u8] {
    let mut binary_string: &[u8] = initial_seed;
    let mut result: &[u8] = b"";

    for i in 0..10 {
        let mut first_half: &[u8] = &binary_string[..(binary_string.len()) / 2];
        let mut second_half: &[u8] = &binary_string[(binary_string.len()) / 2..];
        binary_string = h_fn(first_half, second_half);
        result += binary_string[-1];
        binary_string = binary_string[..-1];
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

fn controller() {

}

fn main() {
    controller();
}