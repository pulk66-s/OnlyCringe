extern crate bcrypt;

use bcrypt::{hash, verify};

pub fn crypt(password: String) -> String {
    let hashed: String = match hash(password, 6) {
        Ok(x) => x,
        Err(_) => "".to_string(),
    };
    return hashed;
}

pub fn ematch(crypted: &String, to_verify: &String) -> bool {
    return match verify(to_verify, &crypted) {
        Ok(x) => x,
        Err(_) => false,
    };
}