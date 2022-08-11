use crate::data::User;
use rocket::serde::uuid::Uuid;
use OC_utils::{api, database};

pub async fn profile_picture(uuid: &Uuid) -> bool {
    let url = format!("http://localhost:5000/api/image/profile/{}", uuid);
    return match api::delete(&url).await {
        Some(_) => true,
        None => false,
    };
}

pub fn from_obj(user: &User) -> bool {
    if let Some(uuid) = &user.uuid {
        let query = format!("update User set archived=true where uuid='{}'", uuid);
        return database::send(&query);
    } else {
        return false;
    }
}
