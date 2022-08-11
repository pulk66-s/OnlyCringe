use crate::service::{chat, forum};
use rocket::delete;
use rocket::response::status::{Accepted, BadRequest};
use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;
use OC_user::data::User;

#[delete("/<name>", rank = 2)]
pub fn delete_by_name(name: String) -> Result<Accepted<String>, BadRequest<String>> {
    return match forum::delete::by_name(name) {
        true => Ok(Accepted(Some("Forum deleted".to_string()))),
        false => Err(BadRequest(Some("Forum not deleted".to_string()))),
    };
}

#[delete("/<uuid>", rank = 1)]
pub fn delete_by_uuid(uuid: Uuid) -> Result<Accepted<String>, BadRequest<String>> {
    return match forum::delete::by_uuid(uuid) {
        true => Ok(Accepted(Some("Forum deleted".to_string()))),
        false => Err(BadRequest(Some("Forum not deleted".to_string()))),
    };
}

#[delete("/chat/<name>", rank = 2)]
pub fn delete_chat_by_name(name: String) -> Result<Accepted<String>, BadRequest<String>> {
    return match chat::delete::by_name(name) {
        true => Ok(Accepted(Some("Chat deleted".to_string()))),
        false => Err(BadRequest(Some("Chat not deleted".to_string()))),
    };
}

#[delete("/chat/<uuid>", rank = 1)]
pub fn delete_chat_by_uuid(uuid: Uuid) -> Result<Accepted<String>, BadRequest<String>> {
    return match chat::delete::by_uuid(uuid) {
        true => Ok(Accepted(Some("Chat deleted".to_string()))),
        false => Err(BadRequest(Some("Chat not deleted".to_string()))),
    };
}

#[delete("/sub/<uuid>", data = "<data>")]
pub fn delete_forum_sub(
    uuid: Uuid,
    data: Json<User>,
) -> Result<Accepted<String>, BadRequest<String>> {
    let user = data.into_inner();
    return match forum::delete::sub(uuid, &user) {
        true => Ok(Accepted(Some("User unsubbed".to_string()))),
        false => Err(BadRequest(Some("User not unsubbed".to_string()))),
    };
}
