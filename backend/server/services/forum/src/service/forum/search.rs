use crate::data::Forum;
use crate::service::forum;
use OC_utils::database;

pub fn by_name(name: &String) -> Option<Vec<Forum>> {
    let query = format!("select * from Forum where name like'{}%' limit 20", name);

    return match database::get(&query) {
        Some(x) if x.len() > 0 => Some(forum::get::parse_from_db(x)),
        _ => None,
    };
}
