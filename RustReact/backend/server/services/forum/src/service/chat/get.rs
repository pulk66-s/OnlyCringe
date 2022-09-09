use crate::data::{Chat, Forum};
use crate::service::chat;
use mysql::{from_row, Result, Row};
use rocket::serde::uuid::Uuid;
use std::collections::HashMap;
use OC_user::data::User;
use OC_user::service::get;
use OC_utils::database;

fn parse_from_db(rows: Vec<Result<Row>>) -> Vec<Chat> {
    let mut res = vec![];
    for row in rows {
        let r: (String, String, String, String, String, bool, Option<String>) =
            from_row(row.unwrap());
        res.push(Chat {
            uuid: Some(r.0.clone()),
            author: match Uuid::parse_str(&r.1) {
                Ok(x) => get::get_user_by_uuid(&x),
                Err(_) => None,
            },
            content: Some(r.2),
            forum: Some(Forum::new(r.3)),
            creation_date: Some(r.4),
            archived: Some(r.5),
            answers: chat::get::answers(&Chat::new(r.0)),
            answer_to: match r.6 {
                Some(x) => Some(Box::new(Chat::new(x))),
                None => None,
            },
        });
    }
    return res;
}

pub fn get_keys(chat: &Chat) -> HashMap<String, String> {
    let mut keys = HashMap::new();
    if let Some(uuid) = chat.uuid.clone() {
        keys.insert("uuid".to_string(), uuid);
    }
    if let Some(content) = chat.content.clone() {
        keys.insert("content".to_string(), content);
    }
    if let Some(forum) = chat.forum.clone() {
        if let Some(id) = forum.uuid {
            keys.insert("forum_id".to_string(), id);
        }
    }
    if let Some(creation_date) = chat.creation_date.clone() {
        keys.insert("creation_date".to_string(), creation_date);
    }
    if let Some(author) = chat.author.clone() {
        if let Some(id) = author.uuid {
            keys.insert("author_id".to_string(), id);
        }
    }
    if let Some(answer_to) = chat.answer_to.clone() {
        if let Some(id) = answer_to.uuid {
            keys.insert("answer_to".to_string(), id);
        }
    }
    return keys;
}

pub fn by_author(user: User) -> Option<Vec<Chat>> {
    if let Some(uuid) = user.uuid {
        let query = format!("select * from Chat where author_id='{}'", uuid);
        return match database::get(&query) {
            Some(x) => Some(parse_from_db(x)),
            None => None,
        };
    }
    return None;
}

pub fn by_uuid(uuid: &Uuid) -> Option<Chat> {
    let query = format!("select * from Chat where uuid='{}'", uuid);
    return match database::get(&query) {
        Some(x) => Some(parse_from_db(x)[0].clone()),
        None => None,
    };
}

pub fn by_forum(forum: Forum) -> Option<Vec<Chat>> {
    if let Some(uuid) = forum.uuid {
        let query = format!(
            "select * from Chat where forum_id='{}' and answer_to is NULL",
            uuid
        );
        return match database::get(&query) {
            Some(x) => Some(parse_from_db(x)),
            None => None,
        };
    }
    return None;
}

pub fn answers(chat: &Chat) -> Option<Vec<Chat>> {
    if let Some(uuid) = &chat.uuid {
        let query = format!("select * from Chat where answer_to='{}'", uuid);
        return match database::get(&query) {
            Some(x) => Some(parse_from_db(x)),
            None => None,
        };
    }
    return None;
}
