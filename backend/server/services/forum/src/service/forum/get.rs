use crate::data::Forum;
use crate::service::{chat, forum};
use mysql::{from_row, Result, Row};
use rocket::serde::uuid::Uuid;
use std::collections::HashMap;
use OC_user::data::User;
use OC_user::service::get;
use OC_utils::database;

pub fn parse_from_db(rows: Vec<Result<Row>>) -> Vec<Forum> {
    let mut res = vec![];
    for row in rows {
        let r: (String, String, String, bool, String, String) = from_row(row.unwrap());
        res.push(Forum {
            uuid: Some(r.0.clone()),
            name: Some(r.1),
            description: Some(r.2),
            archived: Some(r.3),
            author: match Uuid::parse_str(&r.4) {
                Ok(x) => get::get_user_by_uuid(&x),
                Err(_) => None,
            },
            chats: chat::get::by_forum(Forum::new(r.0.clone())),
            subs: match Uuid::parse_str(&r.0) {
                Ok(x) => user_subbed(&x),
                Err(_) => None,
            },
            creation_date: Some(r.5),
        });
    }
    return res;
}

pub fn get_keys(forum: &Forum) -> HashMap<String, String> {
    let mut res = HashMap::new();
    if let Some(uuid) = forum.uuid.clone() {
        res.insert("uuid".to_string(), uuid);
    }
    if let Some(name) = forum.name.clone() {
        res.insert("name".to_string(), name);
    }
    if let Some(description) = forum.description.clone() {
        res.insert("description".to_string(), description);
    }
    if let Some(archived) = forum.archived {
        res.insert("archived".to_string(), format!("{}", archived));
    }
    if let Some(user) = forum.author.clone() {
        if let Some(uuid) = user.uuid {
            res.insert("author_id".to_string(), uuid);
        }
    }
    return res;
}

pub fn all() -> Option<Vec<Forum>> {
    let query = "select * from Forum";
    return match database::get(query) {
        Some(x) => Some(parse_from_db(x)),
        None => None,
    };
}

pub fn by_name(name: String) -> Option<Forum> {
    let query = format!("select * from Forum where name='{}'", name);
    return match database::get(&query) {
        Some(x) if x.len() > 0 => Some(parse_from_db(x)[0].clone()),
        _ => None,
    };
}

pub fn by_uuid(uuid: &Uuid) -> Option<Forum> {
    let id = uuid.to_string();
    let query = format!("select * from Forum where uuid='{}'", id);
    return match database::get(&query) {
        Some(x) if x.len() > 0 => Some(parse_from_db(x)[0].clone()),
        _ => None,
    };
}

pub fn user_subbed(uuid: &Uuid) -> Option<Vec<User>> {
    let forum_request = format!("select archived from Forum where uuid='{}'", uuid);
    match database::get(&forum_request) {
        Some(x) if x.len() > 0 => {
            for row in x {
                let r: bool = from_row(row.unwrap());
                if r {
                    return None;
                }
            }
        }
        _ => return None,
    };
    let id_request = format!("select uid from Subs where fid='{}'", uuid);
    let users = match database::get(&id_request) {
        Some(x) if x.len() > 0 => {
            let mut res = vec![];
            for row in x {
                let r: String = from_row(row.unwrap());
                if let Ok(uuid) = Uuid::parse_str(&r) {
                    if let Some(user) = get::get_user_by_uuid(&uuid) {
                        if let Some(archived) = user.archived {
                            if !archived {
                                res.push(user);
                            }
                        }
                    }
                }
            }
            res
        }
        _ => return None,
    };
    return Some(users);
}

pub fn user_forum_subbed(uuid: &Uuid) -> Option<Vec<Forum>> {
    let user_request = format!("select archived from User where uuid='{}'", uuid);
    match database::get(&user_request) {
        Some(x) if x.len() > 0 => {
            for row in x {
                let r: bool = from_row(row.unwrap());
                if r {
                    return None;
                }
            }
        }
        _ => return None,
    };
    let id_request = format!("select fid from Subs where uid='{}'", uuid);
    let forums = match database::get(&id_request) {
        Some(x) => {
            let mut res = vec![];
            for row in x {
                let r: String = from_row(row.unwrap());
                if let Ok(uuid) = Uuid::parse_str(&r) {
                    if let Some(forum) = forum::get::by_uuid(&uuid) {
                        if let Some(archived) = forum.archived {
                            if !archived {
                                res.push(forum);
                            }
                        }
                    }
                }
            }
            res
        }
        None => return None,
    };
    return Some(forums);
}

pub fn is_sub(user: &User, forum: &Forum) -> bool {
    let subs;
    if let Some(x) = &forum.subs {
        subs = x;
    } else {
        return false;
    }
    for sub in subs {
        match (sub.uuid.clone(), user.uuid.clone()) {
            (Some(sid), Some(uid)) => {
                if sid == uid {
                    return true;
                }
            }
            _ => {}
        }
    }
    return false;
}
