use rand::Rng;

fn main() {
    let message: &str = "Encrypt, this message";
    let key: &[u8] = b"napster";
    let length_message: usize = message.chars().count();
    let password = random_generator(key, length_message);

    // Convert to bytes
    let bytes_message = convert_bytes(message);
    let bytes_password = convert_bytes(password.as_str());

    // Apply xor
    let xor_message = xor(bytes_message, bytes_password);

    // Convert xor to String
    let string_message = convert_string(xor_message);

    println!("XOR String: {:?}", string_message);

    // Convert Xor message to bytes

    let decrypt_message = xor(string_message.as_bytes(), bytes_password);
    let decrypt_string_message = convert_string(decrypt_message);

    println!("Decrypt message: {:?}", decrypt_string_message);
}

fn convert_bytes(message: &str) -> &[u8]{
   let message_bytes = message.as_bytes();
    message_bytes
}

fn xor(message: &[u8], key: &[u8]) -> Vec<u8> {
    let mut xor: Vec<u8> = Vec::new();

    for n in 0.. message.len(){
        xor.push(message[n] ^ key[n]);
    }

    xor.to_vec()
}

fn convert_string(message: Vec<u8>) -> String{
    String::from_utf8(message).unwrap()
}

fn random_generator(key: &[u8], length: usize) -> String {
    let mut rng = rand::thread_rng();

    let password: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0, key.len());
            key[idx] as char
        })
        .collect();

    password
}