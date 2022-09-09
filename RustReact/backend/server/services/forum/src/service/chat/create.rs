use crate::data::Chat;
use crate::service::chat;
use OC_utils::database;

pub fn from_obj(chat: Chat) -> bool {
    let key_map = chat::get::get_keys(&chat);
    let mut keys = vec![];
    let mut values = vec![];
    for (i, j) in key_map {
        keys.push(i);
        values.push(format!("'{}'", j));
    }
    let query = format!(
        "insert into Chat ({}) values ({})",
        keys.join(", "),
        values.join(", ")
    );
    return database::send(&query);
}
