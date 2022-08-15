use crate::data::User;
use crate::service::{delete, friend, get};
use rocket::delete;
use rocket::response::status::{Accepted, BadRequest};
use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;

#[delete("/profile/<uuid>")]
pub async fn delete_profile(uuid: Uuid) -> Result<Accepted<String>, BadRequest<String>> {
    let user = match get::get_user_by_uuid(&uuid) {
        Some(x) => x,
        None => return Err(BadRequest(Some("User not found".to_string()))),
    };
    return match user.uuid {
        Some(x) => {
            return match Uuid::parse_str(&x) {
                Ok(x) => match delete::profile_picture(&x).await {
                    true => Ok(Accepted(Some("Profile deleted".to_string()))),
                    false => Err(BadRequest(Some("Profile not deleted".to_string()))),
                },
                Err(_) => return Err(BadRequest(Some("Error while parsing uuid".to_string()))),
            };
        }
        None => Err(BadRequest(Some("User not found".to_string()))),
    };
}

#[delete("/friend/<uuid>", data = "<user>")]
pub fn delete_friend(uuid: Uuid, user: Json<User>) -> Result<Accepted<String>, BadRequest<String>> {
    let user = user.into_inner();
    let friend = match get::get_user_by_uuid(&uuid) {
        Some(x) => x,
        None => return Err(BadRequest(Some("User not found".to_string()))),
    };
    return match friend::delete(&user, &friend) {
        true => Ok(Accepted(Some("Friend deleted".to_string()))),
        false => Err(BadRequest(Some("Friend not deleted".to_string()))),
    };
}

#[delete("/<name>", rank = 2)]
pub fn delete_by_name(name: String) -> Result<Accepted<String>, BadRequest<String>> {
    let user = match get::get_user_by_name(&name) {
        Some(x) => x,
        None => return Err(BadRequest(Some("User not deleted".to_string()))),
    };
    if user.archived == Some(true) {
        return Err(BadRequest(Some("User not deleted".to_string())));
    }
    return match delete::from_obj(&user) {
        true => Ok(Accepted(Some("User deleted".to_string()))),
        false => Err(BadRequest(Some("User not deleted".to_string()))),
    };
}

#[delete("/<uuid>", rank = 1)]
pub fn delete_by_uuid(uuid: Uuid) -> Result<Accepted<String>, BadRequest<String>> {
    let user = match get::get_user_by_uuid(&uuid) {
        Some(x) => x,
        None => return Err(BadRequest(Some("User not deleted".to_string()))),
    };
    if user.archived == Some(true) {
        return Err(BadRequest(Some("User not deleted".to_string())));
    }
    return match delete::from_obj(&user) {
        true => Ok(Accepted(Some("User deleted".to_string()))),
        false => Err(BadRequest(Some("User not deleted".to_string()))),
    };
}
