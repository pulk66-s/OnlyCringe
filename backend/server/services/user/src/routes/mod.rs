mod delete;
mod get;
mod post;
mod put;

use rocket::{get, routes, Route};

#[get("/ping")]
pub fn ping() -> String {
    return "pong".to_string();
}

pub fn get_all_routes() -> Vec<Route> {
    return routes![
        ping,
        get::get_all,
        get::get_by_uuid,
        get::get_by_name,
        get::download_profile,
        get::is_friend,
        get::is_friend_asked,
        get::get_friends,
        post::create_route,
        post::upload_profile,
        post::add_friend,
        post::accept_friend,
        post::decline_friend,
        post::login_route,
        post::verify_by_name,
        post::verify_by_uuid,
        put::modify_by_name,
        put::modify_by_uuid,
        delete::delete_profile,
        delete::delete_friend,
        delete::delete_by_name,
        delete::delete_by_uuid
    ];
}
