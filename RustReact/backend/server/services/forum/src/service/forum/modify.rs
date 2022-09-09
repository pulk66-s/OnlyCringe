use crate::data::Forum;
use crate::service::{chat, forum};
use rocket::serde::uuid::Uuid;
use OC_utils::database;

pub fn by_name(name: String, forum: &Forum) -> bool {
    let key_map = forum::get::get_keys(forum);
    let mut query_keys = vec![];
    for (i, j) in key_map {
        if i == "archived" {
            query_keys.push(format!("{}={}", i, j));
        } else {
            query_keys.push(format!("{}='{}'", i, j));
        }
    }
    let query = format!(
        "update Forum set {} where name='{}'",
        query_keys.join(", "),
        name
    );
    return database::send(&query);
}

pub fn by_uuid(uuid: Uuid, forum: &Forum) -> bool {
    let key_map = forum::get::get_keys(forum);
    let mut query_keys = vec![];
    for (i, j) in key_map {
        if i == "archived" {
            query_keys.push(format!("{}={}", i, j));
        } else {
            query_keys.push(format!("{}='{}'", i, j));
        }
    }
    let query = format!(
        "update Forum set {} where uuid='{}'",
        query_keys.join(", "),
        uuid
    );
    if !database::send(&query) {
        return false;
    }
    if let Some(chats) = &forum.chats {
        for chat in chats {
            if let Some(uuid_str) = &chat.uuid {
                if let Ok(uuid) = Uuid::parse_str(&uuid_str) {
                    chat::modify::modify_by_uuid(uuid, chat);
                }
            }
        }
    }
    return true;
}
