use crate::data::User;
use crate::service::get;
use OC_utils::jwt::Token;
use OC_utils::{encrypt, jwt};

fn get_user_jwt_values(user: &User) -> Option<String> {
    if let Some(x) = &user.name {
        return Some(x.clone());
    }
    return None;
}

pub fn verify_jwt(user: &User, token: &Token) -> bool {
    return match get_user_jwt_values(user) {
        Some(x) => jwt::verify(&x, &token.token),
        None => false,
    };
}

pub fn create_jwt(user: &User) -> Option<String> {
    return match get_user_jwt_values(user) {
        Some(x) => Some(jwt::create(&x)),
        None => None,
    };
}

pub fn login(user: &User) -> bool {
    let cloned = user.clone();
    return match (&cloned.name, &cloned.password) {
        (Some(name), Some(pwd)) => {
            let wanted: User = match get::get_user_by_name(name) {
                Some(x) => x,
                None => return false,
            };
            match wanted.password {
                Some(enc) => encrypt::ematch(&enc, pwd),
                None => false,
            }
        }
        _ => false,
    };
}
