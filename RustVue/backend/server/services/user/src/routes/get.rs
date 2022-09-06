use crate::service::{friend, get};
use mysql::serde_json;
use rocket::get;
use rocket::response::status::{Accepted, BadRequest, Conflict, NoContent};
use rocket::serde::uuid::Uuid;

#[get("/")]
pub fn get_all() -> Result<Accepted<String>, Conflict<String>> {
    return match get::get_user(None) {
        Some(x) => match serde_json::to_string(&x) {
            Ok(x) => Ok(Accepted(Some(x))),
            Err(_) => Err(Conflict(Some("Error while parsing into json".to_string()))),
        },
        None => Err(Conflict(Some("Error while fetching users".to_string()))),
    };
}

#[get("/<uuid>", rank = 1)]
pub fn get_by_uuid(uuid: Uuid) -> Result<Accepted<String>, NoContent> {
    return match get::get_user_by_uuid(&uuid) {
        Some(x) => match serde_json::to_string(&x) {
            Ok(x) => Ok(Accepted(Some(x))),
            Err(_) => Err(NoContent),
        },
        None => Err(NoContent),
    };
}

#[get("/<name>", rank = 2)]
pub fn get_by_name(name: String) -> Result<String, NoContent> {
    return match get::get_user_by_name(&name) {
        Some(x) => match serde_json::to_string(&x) {
            Ok(x) => Ok(x),
            Err(_) => Err(NoContent),
        },
        None => Err(NoContent),
    };
}

#[get("/profile/<uuid>")]
pub async fn download_profile(uuid: Uuid) -> Result<Accepted<String>, BadRequest<String>> {
    let user = match get::get_user_by_uuid(&uuid) {
        Some(x) => x,
        None => return Err(BadRequest(Some("User not found".to_string()))),
    };
    return match user.uuid {
        Some(x) => {
            return match Uuid::parse_str(&x) {
                Ok(x) => match get::get_profile_picture(&x).await {
                    Some(x) => Ok(Accepted(Some(x))),
                    None => Err(BadRequest(Some("Profile picture not found1".to_string()))),
                },
                Err(_) => return Err(BadRequest(Some("Profile picture not found2".to_string()))),
            };
        }
        None => Err(BadRequest(Some("User not found".to_string()))),
    };
}

#[get("/friend/is/<uid>/<fid>")]
pub fn is_friend(uid: Uuid, fid: Uuid) -> Result<String, NoContent> {
    let user = match get::get_user_by_uuid(&uid) {
        Some(x) => x,
        None => return Err(NoContent),
    };
    let friend = match get::get_user_by_uuid(&fid) {
        Some(x) => x,
        None => return Err(NoContent),
    };
    return match get::is_friend(&user, &friend) {
        true => Ok("they are friends".to_string()),
        false => Err(NoContent),
    };
}

#[get("/friend/asked/<uid>/<fid>")]
pub fn is_friend_asked(uid: Uuid, fid: Uuid) -> Result<String, NoContent> {
    let user = match get::get_user_by_uuid(&uid) {
        Some(x) => x,
        None => return Err(NoContent),
    };
    let friend = match get::get_user_by_uuid(&fid) {
        Some(x) => x,
        None => return Err(NoContent),
    };
    return match get::is_friend_asked(&user, &friend) {
        true => Ok("already asked".to_string()),
        false => Err(NoContent),
    };
}

#[get("/friend/<uuid>?<status>")]
pub fn get_friends(uuid: Uuid, status: Option<String>) -> Result<Accepted<String>, NoContent> {
    let user = match get::get_user_by_uuid(&uuid) {
        Some(x) => x,
        None => return Err(NoContent),
    };
    let friends = match friend::get_friends(&user, status) {
        Some(x) => x,
        None => return Err(NoContent),
    };
    return match serde_json::to_string(&friends) {
        Ok(x) => Ok(Accepted(Some(x))),
        Err(_) => Err(NoContent),
    };
}
