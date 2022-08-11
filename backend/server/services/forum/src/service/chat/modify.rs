use crate::data::Chat;
use crate::service::chat;
use OC_utils::database;
use rocket::serde::uuid::Uuid;

pub fn modify_by_uuid(uuid: Uuid, chat: &Chat) -> bool {
    let key_map = chat::get::get_keys(&chat);
    let mut query_key = vec![];
    for (i, j) in key_map {
        if i == "archived" {
            query_key.push(format!("{}={}", i, j));
        } else {
            query_key.push(format!("{}='{}'", i, j));
        }
    }
    let query = format!(
        "update Chat set {} where uuid='{}'",
        query_key.join(", "),
        uuid
    );
    return database::send(&query);
}
