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
        get::get_by_name,
        get::get_by_uuid,
        get::get_forum_sub,
        get::get_user_forum_subbed_by_name,
        get::get_is_user_sub_to_forum,
        get::search_forum_by_name,
        get::get_chat_answer,
        get::get_chat_by_uuid,
        post::create_forum,
        post::create_chat,
        post::create_forum_sub,
        delete::delete_by_name,
        delete::delete_by_uuid,
        delete::delete_chat_by_name,
        delete::delete_chat_by_uuid,
        delete::delete_forum_sub,
        put::modify_by_name,
        put::modify_by_uuid,
        put::modify_chat_by_uuid
    ];
}
