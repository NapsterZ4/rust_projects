fn main() {
    let bytes_message = convert_binary("Napster");
    let key = convert_binary("Javierr");
    
    let encrypt_message = xor(bytes_message, key);

    let encrypt_message_xor = convert_binary(encrypt_message.as_str());

    let decrypt_message = xor(encrypt_message_xor, key);

    println!("Encrypt is: {:?}", encrypt_message);
    println!("Decrypt is: {:?}", decrypt_message);
}


fn convert_binary(message: &str) -> &[u8]{
   let message_bytes = message.as_bytes();
   return message_bytes;
}


fn xor(message: &[u8], key: &[u8]) -> String{
    let mut xor: Vec<u8> = Vec::new();

    for n in 0.. message.len(){
        xor.push(message[n] ^ key[n]);
    }

    return String::from_utf8(xor).unwrap();
}
