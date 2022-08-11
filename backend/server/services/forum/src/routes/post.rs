use crate::data::{Chat, Forum};
use crate::service::{chat, forum};
use rocket::post;
use rocket::response::status::{Accepted, BadRequest};
use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;
use OC_user::data::User;

#[post("/", data = "<data>")]
pub fn create_forum(data: Json<Forum>) -> Result<Accepted<String>, BadRequest<String>> {
    let forum = data.into_inner();
    return match forum::create::from_obj(&forum) {
        true => Ok(Accepted(Some("Forum created".to_string()))),
        false => Err(BadRequest(Some("Forum not created".to_string()))),
    };
}

#[post("/chat", data = "<data>")]
pub fn create_chat(data: Json<Chat>) -> Result<Accepted<String>, BadRequest<String>> {
    let chat = data.into_inner();
    return match chat::create::from_obj(chat) {
        true => Ok(Accepted(Some("Chat created".to_string()))),
        false => Err(BadRequest(Some("Chat not created".to_string()))),
    };
}

#[post("/sub/<uuid>", data = "<data>")]
pub fn create_forum_sub(uuid: Uuid, data: Json<User>) -> Result<Accepted<String>, BadRequest<String>> {
    let user = data.into_inner();
    return match forum::create::sub(uuid, &user) {
        true => Ok(Accepted(Some("User sub into forum".to_string()))),
        false => Err(BadRequest(Some("User not subbed".to_string()))),
    };
}
