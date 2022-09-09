use rocket::serde::uuid::Uuid;
use OC_user::data::User;
use OC_utils::database;

pub fn by_name(name: String) -> bool {
    let query = format!("update Forum set archived=true where name='{}'", name);
    return database::send(&query);
}

pub fn by_uuid(uuid: Uuid) -> bool {
    let query = format!("update Forum set archived=true where uuid='{}'", uuid);
    return database::send(&query);
}

pub fn sub(uuid: Uuid, user: &User) -> bool {
    return match &user.uuid {
        Some(x) => {
            let request = format!("delete from Subs where uid='{}' and fid='{}'", x, uuid);
            return database::send(&request);
        }
        None => false,
    };
}
