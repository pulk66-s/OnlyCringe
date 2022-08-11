use crate::data::Forum;
use crate::service::forum;
use rocket::serde::uuid::Uuid;
use OC_user::data::User;
use OC_utils::database;

pub fn from_obj(forum: &Forum) -> bool {
    let key_map = forum::get::get_keys(forum);
    let mut keys = vec![];
    let mut values = vec![];
    for (i, j) in key_map {
        keys.push(i);
        values.push(format!("'{}'", j));
    }
    let query = format!(
        "insert into Forum ({}) values ({})",
        keys.join(", "),
        values.join(", ")
    );
    return database::send(&query);
}

pub fn sub(uuid: Uuid, user: &User) -> bool {
    return match &user.uuid {
        Some(x) => {
            let request = format!("insert into Subs (uid, fid) values ('{}', '{}')", x, uuid);
            return database::send(&request);
        }
        None => false,
    };
}
