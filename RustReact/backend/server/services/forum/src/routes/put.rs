use crate::data::{Chat, Forum};
use crate::service::{chat, forum};
use rocket::put;
use rocket::response::status::{Accepted, BadRequest};
use rocket::serde::{json::Json, uuid::Uuid};

#[put("/<name>", rank = 2, data = "<data>")]
pub fn modify_by_name(
    name: String,
    data: Json<Forum>,
) -> Result<Accepted<String>, BadRequest<String>> {
    let forum = data.into_inner();
    return match forum::modify::by_name(name, &forum) {
        true => Ok(Accepted(Some("Forum modified".to_string()))),
        false => Err(BadRequest(Some("Forum not modified".to_string()))),
    };
}

#[put("/<uuid>", rank = 1, data = "<data>")]
pub fn modify_by_uuid(
    uuid: Uuid,
    data: Json<Forum>,
) -> Result<Accepted<String>, BadRequest<String>> {
    let forum = data.into_inner();
    return match forum::modify::by_uuid(uuid, &forum) {
        true => Ok(Accepted(Some("Forum modified".to_string()))),
        false => Err(BadRequest(Some("Forum not modified".to_string()))),
    };
}

#[put("/chat/<uuid>", data = "<data>")]
pub fn modify_chat_by_uuid(
    uuid: Uuid,
    data: Json<Chat>,
) -> Result<Accepted<String>, BadRequest<String>> {
    let chat = data.into_inner();
    return match chat::modify::modify_by_uuid(uuid, &chat) {
        true => Ok(Accepted(Some("Chat modified".to_string()))),
        false => Err(BadRequest(Some("Chat not modified".to_string()))),
    };
}
