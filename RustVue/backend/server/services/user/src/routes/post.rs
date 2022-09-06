use crate::data::User;
use crate::service::{create, friend, get, login};
use rocket::data::{Data, ToByteUnit};
use rocket::post;
use rocket::response::status::{Accepted, BadRequest};
use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;
use OC_utils::jwt;

#[post("/", data = "<user>")]
pub fn create_route(user: Json<User>) -> Result<Accepted<String>, BadRequest<String>> {
    let user_json: User = user.into_inner();
    return match create::from_obj(&user_json) {
        true => Ok(Accepted(Some("User created".to_string()))),
        false => Err(BadRequest(Some("User not created".to_string()))),
    };
}

#[post("/profile/<uuid>", data = "<data>")]
pub async fn upload_profile(
    uuid: Uuid,
    data: Data<'_>,
) -> Result<Accepted<String>, BadRequest<String>> {
    let user = match get::get_user_by_uuid(&uuid) {
        Some(x) => x,
        None => return Err(BadRequest(Some("User not found".to_string()))),
    };
    let body = match data.open(1024.kibibytes()).into_bytes().await {
        Ok(x) => x.into_inner(),
        Err(_) => return Err(BadRequest(Some("Error while reading body".to_string()))),
    };
    return match user.uuid {
        Some(x) => match Uuid::parse_str(&x) {
            Ok(x) => match create::profile_picture(&x, body).await {
                true => Ok(Accepted(Some("Profile uploaded".to_string()))),
                false => Err(BadRequest(Some("Profile not uploaded".to_string()))),
            },
            Err(_) => Err(BadRequest(Some("Error while parsing uuid".to_string()))),
        },
        None => Err(BadRequest(Some("User not found".to_string()))),
    };
}

#[post("/friend/<fid>", data = "<user>")]
pub fn add_friend(fid: Uuid, user: Json<User>) -> Result<String, BadRequest<String>> {
    let friend = match get::get_user_by_uuid(&fid) {
        Some(x) => x,
        None => return Err(BadRequest(Some("User not found".to_string()))),
    };
    let user = user.into_inner();
    return match friend::create(&user, &friend) {
        true => Ok("Friend added".to_string()),
        false => Err(BadRequest(Some("Friend not added".to_string()))),
    };
}

#[post("/friend/accept/<uuid>", data = "<user>")]
pub fn accept_friend(uuid: Uuid, user: Json<User>) -> Result<Accepted<String>, BadRequest<String>> {
    let user = user.into_inner();
    let friend = match get::get_user_by_uuid(&uuid) {
        Some(x) => x,
        None => return Err(BadRequest(Some("User not found".to_string()))),
    };
    return match get::accept_friend(&user, &friend) {
        true => Ok(Accepted(Some("Friend accepted".to_string()))),
        false => Err(BadRequest(Some("Friend not accepted".to_string()))),
    };
}

#[post("/friend/decline/<uuid>", data = "<user>")]
pub fn decline_friend(
    uuid: Uuid,
    user: Json<User>,
) -> Result<Accepted<String>, BadRequest<String>> {
    let user = user.into_inner();
    let friend = match get::get_user_by_uuid(&uuid) {
        Some(x) => x,
        None => return Err(BadRequest(Some("User not found".to_string()))),
    };
    return match get::decline_friend(&user, &friend) {
        true => Ok(Accepted(Some("Friend declined".to_string()))),
        false => Err(BadRequest(Some("Friend not declined".to_string()))),
    };
}

#[post("/jwt/login", data = "<data>")]
pub fn login_route(data: Json<User>) -> Result<Accepted<String>, BadRequest<String>> {
    let user: User = data.into_inner();
    return match login::login(&user) {
        true => match login::create_jwt(&user) {
            Some(x) => Ok(Accepted(Some(x))),
            None => Err(BadRequest(Some("Not Accepted".to_string()))),
        },
        false => Err(BadRequest(Some("Not Accepted".to_string()))),
    };
}

#[post("/jwt/verify/<name>", data = "<data>", rank = 2)]
pub fn verify_by_name(
    name: String,
    data: Json<jwt::Token>,
) -> Result<Accepted<String>, BadRequest<String>> {
    let token = data.into_inner();
    let user = match get::get_user_by_name(&name) {
        Some(x) => x,
        None => return Err(BadRequest(Some("user not existing".to_string()))),
    };
    if let Some(archived) = user.archived {
        if archived {
            return Err(BadRequest(Some("Wrong token".to_string())));
        }
    }
    return match login::verify_jwt(&user, &token) {
        true => Ok(Accepted(Some("Accepted".to_string()))),
        false => Err(BadRequest(Some("Wrong token".to_string()))),
    };
}

#[post("/jwt/verify/<uuid>", data = "<data>", rank = 1)]
pub fn verify_by_uuid(
    uuid: Uuid,
    data: Json<jwt::Token>,
) -> Result<Accepted<String>, BadRequest<String>> {
    let token = data.into_inner();
    let user = match get::get_user_by_uuid(&uuid) {
        Some(x) => x,
        None => return Err(BadRequest(Some("user not existing".to_string()))),
    };
    return match login::verify_jwt(&user, &token) {
        true => Ok(Accepted(Some("Accepted".to_string()))),
        false => Err(BadRequest(Some("Wrong token".to_string()))),
    };
}
