fn main() {
    let bytes_message = convert_bytes("Napster");
    let key = convert_bytes("Javierr");
    
    let encrypt_message = xor(bytes_message, key);
    let string_message = convert_string(encrypt_message);

    println!("Convert String: {:?}", string_message);

    let encrypt_message_xor = convert_bytes(&*string_message);

    let decrypt_message = xor(encrypt_message_xor, key);
    let decrypt_string_message = convert_string(decrypt_message);

    println!("Convert String: {:?}", decrypt_string_message);
}

fn convert_bytes(message: &str) -> &[u8]{
   let message_bytes = message.as_bytes();
   return message_bytes;
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