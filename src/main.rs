use std::*;
use std::ptr::null;
use std::arch::x86::_mm256_blend_epi16;

const SEED_SIZE: i32 = 16;
const GENERATOR: usize = 223;
const MODULUS: usize = 36389;
const SEED: &str = "1001100001110001";

// const function_l: = |x| ((x.pow(2) - (2*x)) + 1);


fn main() {
    controller();
}

fn l(k: i32) -> i32 {
    (k.pow(2) - 2(k) + 7)
}

fn h_fn(first_half: &str, second_half: &str){
    let mut mod_exp: i8 = 2.pow(first_half.parse().unwrap()) % MODULUS;
    let mut hard_code_bit: i32 = 0;

    for i in first_half {
        hard_code_bit = (hard_code_bit ^ ((first_half[i]) & (second_half[i]))) % 2;
    }

    mod_exp; second_half; str(hard_code_bit)
}


fn g_fn(initial_seed: &str){
    let mut binary_string: &str = initial_seed;
    let mut result: &str = "";
    let mut first_half: &str = "";
    let mut second_half: &str = "";

    for i in l(SEED_SIZE){
        first_half = &binary_string[..(binary_string.len()) / 2];
        second_half = &binary_string[(binary_string.len()) / 2 ..];
        binary_string = h_fn(first_half, second_half);
    }
}

fn xor(message: &[u8], key: &[u8]) -> Vec<u8> {
    let mut xor: Vec<u8> = Vec::new();

    for n in 0.. message.len(){
        xor.push(message[n] ^ key[n]);
    }

    xor.to_vec()
}




fn controller()  {

}