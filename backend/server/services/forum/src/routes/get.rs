use crate::service::{forum, chat};
use mysql::serde_json;
use rocket::get;
use rocket::response::status::{Accepted, BadRequest, NoContent};
use rocket::serde::uuid::Uuid;
use OC_user::service::get;

#[get("/")]
pub fn get_all() -> Result<Accepted<String>, NoContent> {
    return match forum::get::all() {
        Some(x) => match serde_json::to_string(&x) {
            Ok(x) => Ok(Accepted(Some(x))),
            Err(_) => Err(NoContent),
        },
        None => Err(NoContent),
    };
}

#[get("/<name>", rank = 2)]
pub fn get_by_name(name: String) -> Result<Accepted<String>, NoContent> {
    return match forum::get::by_name(name) {
        Some(x) => match serde_json::to_string(&x) {
            Ok(x) => Ok(Accepted(Some(x))),
            Err(_) => Err(NoContent),
        },
        None => Err(NoContent),
    };
}

#[get("/<uuid>", rank = 1)]
pub fn get_by_uuid(uuid: Uuid) -> Result<Accepted<String>, NoContent> {
    return match forum::get::by_uuid(&uuid) {
        Some(x) => match serde_json::to_string(&x) {
            Ok(x) => Ok(Accepted(Some(x))),
            Err(_) => Err(NoContent),
        },
        None => Err(NoContent),
    };
}

#[get("/sub/<uuid>")]
pub fn get_forum_sub(uuid: Uuid) -> Result<Accepted<String>, NoContent> {
    return match forum::get::user_subbed(&uuid) {
        Some(x) => match serde_json::to_string(&x) {
            Ok(x) => Ok(Accepted(Some(x))),
            Err(_) => Err(NoContent),
        },
        None => Err(NoContent),
    };
}

#[get("/sub/user/<name>")]
pub fn get_user_forum_subbed_by_name(name: String) -> Result<Accepted<String>, NoContent> {
    let user = match get::get_user_by_name(&name) {
        Some(x) => x,
        None => return Err(NoContent),
    };
    let uuid;
    if let Some(user_uuid) = user.uuid {
        uuid = match Uuid::parse_str(&user_uuid) {
            Ok(x) => x,
            Err(_) => return Err(NoContent),
        }
    } else {
        return Err(NoContent);
    }
    return match forum::get::user_forum_subbed(&uuid) {
        Some(x) => match serde_json::to_string(&x) {
            Ok(x) => Ok(Accepted(Some(x))),
            Err(_) => Err(NoContent),
        },
        None => Err(NoContent),
    };
}

#[get("/issub/<uid>/<fid>")]
pub fn get_is_user_sub_to_forum(
    uid: Uuid,
    fid: Uuid,
) -> Result<Accepted<String>, BadRequest<String>> {
    let user = match get::get_user_by_uuid(&uid) {
        Some(x) => x,
        None => return Err(BadRequest(Some("User not existing".to_string()))),
    };
    let forum = match forum::get::by_uuid(&fid) {
        Some(x) => x,
        None => return Err(BadRequest(Some("Forum not existing".to_string()))),
    };
    return match forum::get::is_sub(&user, &forum) {
        true => Ok(Accepted(Some("User is subbed".to_string()))),
        false => Err(BadRequest(Some("Use is not subbed".to_string()))),
    };
}

#[get("/search/<name>")]
pub fn search_forum_by_name(name: String) -> Result<Accepted<String>, NoContent> {
    return match forum::search::by_name(&name) {
        Some(x) => match serde_json::to_string(&x) {
            Ok(x) => Ok(Accepted(Some(x))),
            Err(_) => Err(NoContent),
        },
        None => Err(NoContent),
    };
}

#[get("/chat/answer/<uuid>")]
pub fn get_chat_answer(uuid: Uuid) -> Result<Accepted<String>, NoContent> {
    let chat = match chat::get::by_uuid(&uuid) {
        Some(x) => x,
        None => return Err(NoContent),
    };
    return match chat.answers {
        Some(x) => match serde_json::to_string(&x) {
            Ok(x) => Ok(Accepted(Some(x))),
            Err(_) => Err(NoContent),
        },
        None => Err(NoContent),
    };
}

#[get("/chat/<uuid>")]
pub fn get_chat_by_uuid(uuid: Uuid) -> Result<Accepted<String>, NoContent> {
    let chat = match chat::get::by_uuid(&uuid) {
        Some(x) => x,
        None => return Err(NoContent),
    };
    return match serde_json::to_string(&chat) {
        Ok(x) => Ok(Accepted(Some(x))),
        Err(_) => Err(NoContent),
    };
}