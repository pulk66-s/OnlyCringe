use crate::data::User;
use crate::service::convert;
use mysql::from_row;
use rocket::serde::uuid::Uuid;
use OC_utils::database;

pub fn create(user: &User, friend: &User) -> bool {
    let uid = match &user.uuid {
        Some(x) => x,
        None => return false,
    };
    let fid = match &friend.uuid {
        Some(x) => x,
        None => return false,
    };
    let query = format!(
        "insert into Friend (uid, fid) values ('{}', '{}')",
        uid, fid
    );
    return database::send(&query);
}

pub fn delete(user: &User, friend: &User) -> bool {
    let uid = match &user.uuid {
        Some(x) => x,
        None => return false,
    };
    let fid = match &friend.uuid {
        Some(x) => x,
        None => return false,
    };
    let query = format!(
        "delete from Friend where uid='{}' and fid='{}'",
        uid, fid
    );
    return database::send(&query);
}

pub fn get_friends(user: &User, status: Option<String>) -> Option<Vec<User>> {
    let mut query: String = "select * from User where ".to_string();
    let mut keys = vec![];
    let uuid = match &user.uuid {
        Some(x) => x,
        None => return None,
    };
    if let Some(status) = status {
        keys.push(format!(
            "uuid in (select fid from Friend where uid='{}' and status='{}')",
            uuid, status
        ));
    } else {
        keys.push(format!(
            "uuid in (select fid from Friend where uid='{}')",
            uuid
        ));
    }
    query.push_str(&keys.join(" and "));
    return match database::get(&query) {
        Some(x) => Some(convert::from_db(x)),
        None => None,
    };
}

pub fn get_friends_by_uuid(uuid: &Uuid) -> Vec<User> {
    let query = format!("select fid from Friend where uid='{}' and status='ACCEPTED'", uuid);
    let mut result: Vec<User> = vec![];
    let rows = database::get(&query);
    if let Some(rows) = rows {
        for row in rows {
            let r: String = from_row(row.unwrap());
            result.push(User {
                uuid: Some(r),
                name: None,
                description: None,
                password: None,
                email: None,
                archived: None,
                creation_date: None,
                role: None,
                friends: None,
            });
        }
    }
    return result;
}