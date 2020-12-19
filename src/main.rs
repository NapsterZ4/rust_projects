use std::*;
use std::convert::TryInto;

const SEED_SIZE: i32 = 16;
const GENERATOR: usize = 223;
const MODULUS: i32 = 36389;
const SEED: &[u8] = "1001100001110001".as_bytes();


fn h_fn<'a>(first_half: &'a[u8], second_half: &'a[u8]) -> Vec<u8> {
    let mod_exp: i32 = 2_i32.pow(first_half.len().try_into().unwrap()) % MODULUS;
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

fn convert_string(message: Vec<u8>) -> String{
    String::from_utf8(message).unwrap()
}

fn concatenate_arrays<'a, T: Clone>(x: &'a[T], y: &'a[T]) -> Vec<T> {
    let mut concat = x.to_vec();
    concat.extend_from_slice(y);

    concat
}

fn g_fn(initial_seed: &[u8]) -> Vec<u8> {
    let mut binary_string: Vec<u8> = initial_seed.to_vec();

    for _i in 0..10 {
        let first_half: &[u8] = &binary_string[..(binary_string.len()) / 2];
        let second_half: &[u8] = &binary_string[(binary_string.len()) / 2..];
        binary_string = h_fn(first_half, second_half);
    }

    binary_string
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