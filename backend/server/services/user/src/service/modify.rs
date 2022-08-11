use crate::data::User;
use OC_utils::{database, encrypt};

pub fn modify_user(user: &User, new_values: &User) -> bool {
    let id = match &user.uuid {
        Some(x) => x,
        None => return false,
    };
    return update_user(new_values, id);
}

fn update_user(user: &User, uuid: &String) -> bool {
    let mut keys = vec![];
    if let Some(uuid) = &user.uuid {
        keys.push(format!("uuid='{}'", uuid));
    }
    if let Some(name) = &user.name {
        keys.push(format!("name='{}'", name));
    }
    if let Some(pwd) = &user.password {
        keys.push(format!("password='{}'", encrypt::crypt(pwd.to_string())));
    }
    if let Some(email) = &user.email {
        keys.push(format!("email='{}'", email));
    }
    if let Some(archived) = user.archived {
        keys.push(format!("archived={}", archived));
    }
    if let Some(role) = &user.role {
        keys.push(format!("role='{}'", role));
    }
    let query = format!("update User set {} where uuid='{}'", keys.join(", "), uuid);
    return database::send(&query);
}
