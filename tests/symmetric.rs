use std::str;

use fides::symmetric;

#[test]
fn decryption() {

    let msg: &str = "Hello";

    let key = [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32];

    let msg_bytes = msg.as_bytes().to_vec();
    
    let cipher_msg = symmetric::encrypt(&key, &msg_bytes);

    let plain_msg = symmetric::decrypt(&key, &cipher_msg);

    let decrypted = str::from_utf8(&plain_msg).unwrap();

    assert_eq!(msg, decrypted);

}