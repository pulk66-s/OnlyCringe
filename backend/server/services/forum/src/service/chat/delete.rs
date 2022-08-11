use rocket::serde::uuid::Uuid;
use OC_utils::database;

pub fn by_name(name: String) -> bool {
    let query = format!("update Chat set archived=true where name='{}'", name);
    return database::send(&query);
}

pub fn by_uuid(uuid: Uuid) -> bool {
    let query = format!("update Chat set archived=true where uuid='{}'", uuid);
    return database::send(&query);
}
