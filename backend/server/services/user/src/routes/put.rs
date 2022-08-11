use crate::data::User;
use crate::service::{get, modify};
use rocket::put;
use rocket::response::status::{Accepted, BadRequest};
use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;

#[put("/<name>", data = "<data>", rank = 2)]
pub fn modify_by_name(
    name: String,
    data: Json<User>,
) -> Result<Accepted<String>, BadRequest<String>> {
    let user = match get::get_user_by_name(&name) {
        Some(x) => x,
        None => return Err(BadRequest(Some("User not modified".to_string()))),
    };
    let new_values = data.into_inner();
    return match modify::modify_user(&user, &new_values) {
        true => Ok(Accepted(Some("User modified".to_string()))),
        false => Err(BadRequest(Some("User not modified".to_string()))),
    };
}

#[put("/<uuid>", data = "<data>", rank = 1)]
pub fn modify_by_uuid(
    uuid: Uuid,
    data: Json<User>,
) -> Result<Accepted<String>, BadRequest<String>> {
    let user = match get::get_user_by_uuid(&uuid) {
        Some(x) => x,
        None => return Err(BadRequest(Some("User not modified".to_string()))),
    };
    let new_values = data.into_inner();
    return match modify::modify_user(&user, &new_values) {
        true => Ok(Accepted(Some("User modified".to_string()))),
        false => Err(BadRequest(Some("User not modified".to_string()))),
    };
}
