use crate::data::User;
use crate::service::convert;
use rocket::serde::uuid::Uuid;
use OC_utils::{api, database, encrypt};

fn post_user_in_db(user: &User) -> bool {
    let user_map = convert::to_map(user);
    let mut keys = vec![];
    let mut values = vec![];
    let clone = user.clone();
    for (i, j) in user_map {
        if i == "password" {
            keys.push(i);
            match clone.password {
                Some(ref x) => values.push(format!("'{}'", encrypt::crypt(x.clone()))),
                None => return false,
            };
        } else {
            keys.push(i);
            values.push(j);
        }
    }
    let query: String = format!(
        "insert into User ({}) values ({})",
        keys.join(", "),
        values.join(", ")
    );
    return database::send(&query);
}

pub async fn profile_picture(uuid: &Uuid, file: Vec<u8>) -> bool {
    let url = format!("http://localhost:5000/api/image/profile/{}", uuid);
    return match api::post_file(&url, file).await {
        Some(_) => true,
        None => false,
    };
}

pub fn from_obj(user: &User) -> bool {
    let clone = user.clone();
    return match (clone.name, clone.password, clone.email) {
        (Some(_), Some(_), Some(_)) => post_user_in_db(user),
        _ => return false,
    };
}
