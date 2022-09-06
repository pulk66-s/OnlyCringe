use crate::data::{Role, User};
use crate::service::friend;
use mysql::{from_row, Result, Row};
use rocket::serde::uuid::Uuid;
use std::collections::HashMap;

pub fn to_map(user: &User) -> HashMap<String, String> {
    let mut res: HashMap<String, String> = HashMap::new();
    if let Some(uuid) = &user.uuid {
        res.insert("uuid".to_string(), format!("'{}'", uuid));
    }
    if let Some(name) = &user.name {
        res.insert("name".to_string(), format!("'{}'", name));
    }
    if let Some(pwd) = &user.password {
        res.insert("password".to_string(), format!("'{}'", pwd));
    }
    if let Some(email) = &user.email {
        res.insert("email".to_string(), format!("'{}'", email));
    }
    if let Some(archived) = user.archived {
        res.insert("archived".to_string(), format!("{}", archived));
    }
    if let Some(role) = &user.role {
        res.insert("role".to_string(), format!("'{}'", role));
    }
    if let Some(creation_date) = &user.creation_date {
        res.insert("creation_date".to_string(), format!("'{}'", creation_date));
    }
    if let Some(description) = &user.description {
        res.insert("description".to_string(), format!("'{}'", description));
    }
    return res;
}

pub fn from_db(rows: Vec<Result<Row>>) -> Vec<User> {
    let mut result: Vec<User> = vec![];
    for row in rows {
        let r: (
            String,
            String,
            String,
            String,
            String,
            bool,
            String,
            bool,
            String,
        ) = from_row(row.unwrap());
        result.push(User {
            uuid: Some(r.0.clone()),
            name: Some(r.1),
            description: Some(r.2),
            password: Some(r.3),
            email: Some(r.4),
            archived: Some(r.5),
            creation_date: Some(r.6),
            verified: Some(r.7),
            role: Some(Role::new(&r.8)),
            friends: match Uuid::parse_str(&r.0) {
                Ok(x) => Some(friend::get_friends_by_uuid(&x)),
                Err(_) => None,
            },
        })
    }
    return result;
}
