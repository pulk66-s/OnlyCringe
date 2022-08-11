use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Role {
    USER,
    ADMIN,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum FriendStatus {
    ASKING,
    ACCEPTED,
    DECLINED
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub uuid: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub archived: Option<bool>,
    pub role: Option<Role>,
    pub creation_date: Option<String>,
    pub friends: Option<Vec<User>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Friend {
    pub uuid: Option<String>,
    pub uid: Option<User>,
    pub fid: Option<User>,
    pub status: Option<FriendStatus>,
    pub creation_date: Option<String>,
}

impl Role {
    pub fn new(status: &str) -> Role {
        return match status {
            "ADMIN" => Role::ADMIN,
            _ => Role::USER,
        };
    }
}

impl FriendStatus {
    pub fn new(status: &str) -> FriendStatus {
        return match status {
            "ASKING" => FriendStatus::ASKING,
            "ACCEPTED" => FriendStatus::ACCEPTED,
            _ => FriendStatus::DECLINED,
        };
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "uuid={uuid:?}, name={name:?}, password={password:?}, email={email:?}, archived={archived:?}, role={role:?}",
            uuid=self.uuid,
            name=self.name,
            password=self.password,
            email=self.email,
            archived=self.archived,
            role=self.role
        )
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(
            f,
            "{}",
            match self {
                Role::USER => "USER",
                Role::ADMIN => "ADMIN",
            }
        );
    }
}
