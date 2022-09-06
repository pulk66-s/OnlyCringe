use crate::data::User;
use crate::service::convert;
use mysql::from_row;
use rocket::serde::uuid::Uuid;
use std::collections::HashMap;
use OC_utils::{api, database};

fn get_all_users() -> Option<Vec<User>> {
    let query = "select * from User";
    return match database::get(query) {
        Some(x) => Some(convert::from_db(x)),
        None => None,
    };
}

fn get_user_by_value(by: HashMap<&str, &String>) -> Option<Vec<User>> {
    let mut query: String = "select * from User where ".to_string();
    let mut keys = vec![];
    for (i, j) in by {
        if i == "archived" {
            keys.push(format!("{}={}", i, j));
        } else {
            keys.push(format!("{}='{}'", i, j));
        }
    }
    query.push_str(&keys.join(" and "));
    return match database::get(&query) {
        Some(x) => Some(convert::from_db(x)),
        None => None,
    };
}

pub fn get_user(by: Option<HashMap<&str, &String>>) -> Option<Vec<User>> {
    return match by {
        Some(value) => get_user_by_value(value),
        None => get_all_users(),
    };
}

pub fn get_user_by_name(name: &String) -> Option<User> {
    return match get_user(Some(HashMap::from([("name", name)]))) {
        Some(x) if x.len() > 0 => Some(x[0].clone()),
        _ => None,
    };
}

pub fn get_user_by_uuid(uuid: &Uuid) -> Option<User> {
    let res = get_user(Some(HashMap::from([("uuid", &format!("{}", uuid))])));
    return match res {
        Some(x) if x.len() > 0 => Some(x[0].clone()),
        _ => None,
    };
}

pub async fn get_profile_picture(uuid: &Uuid) -> Option<String> {
    let url = format!("http://localhost:5000/api/image/profile/{}", uuid);
    return api::get(&url).await;
}

pub fn is_friend_asked(user: &User, friend: &User) -> bool {
    let uid = match &user.uuid {
        Some(x) => x,
        None => return false,
    };
    let fid = match &friend.uuid {
        Some(x) => x,
        None => return false,
    };
    let query = format!(
        "select status from Friend where uid='{}' and fid='{}'",
        uid, fid
    );
    let rows = database::get(&query);
    if let Some(rows) = rows {
        for row in rows {
            let r: String = from_row(row.unwrap());
            if r == "ASKING" {
                return true;
            }
        }
    }
    return false;
}

pub fn is_friend(user: &User, friend: &User) -> bool {
    let uid = match &user.uuid {
        Some(x) => x,
        None => return false,
    };
    let fid = match &friend.uuid {
        Some(x) => x,
        None => return false,
    };
    let query = format!(
        "select status from Friend where uid='{}' and fid='{}'",
        uid, fid
    );
    let rows = database::get(&query);
    if let Some(rows) = rows {
        for row in rows {
            let r: String = from_row(row.unwrap());
            if r == "ACCEPTED" {
                return true;
            }
        }
    }
    return false;
}

pub fn decline_friend(user: &User, friend: &User) -> bool {
    let uid = match &user.uuid {
        Some(x) => x,
        None => return false,
    };
    let fid = match &friend.uuid {
        Some(x) => x,
        None => return false,
    };
    let query = format!(
        "update Friend set status='DECLINED' where uid='{}' and fid='{}'",
        uid, fid
    );
    return database::send(&query);
}

pub fn accept_friend(user: &User, friend: &User) -> bool {
    let uid = match &user.uuid {
        Some(x) => x,
        None => return false,
    };
    let fid = match &friend.uuid {
        Some(x) => x,
        None => return false,
    };
    let query = format!(
        "update Friend set status='ACCEPTED' where uid='{}' and fid='{}'",
        uid, fid
    );
    return database::send(&query);
}
