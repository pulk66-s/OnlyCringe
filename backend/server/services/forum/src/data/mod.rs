use serde::{Deserialize, Serialize};
use std::fmt;
use OC_user::data::User;

#[derive(Serialize, Deserialize, Clone)]
pub struct Forum {
    pub uuid: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub archived: Option<bool>,
    pub creation_date: Option<String>,
    pub author: Option<User>,
    pub chats: Option<Vec<Chat>>,
    pub subs: Option<Vec<User>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Chat {
    pub uuid: Option<String>,
    pub content: Option<String>,
    pub forum: Option<Forum>,
    pub creation_date: Option<String>,
    pub author: Option<User>,
    pub archived: Option<bool>,
    pub answers: Option<Vec<Chat>>,
    pub answer_to: Option<Box<Chat>>,
}

impl Forum {
    pub fn new(uuid: String) -> Forum {
        return Forum {
            uuid: Some(uuid),
            name: None,
            description: None,
            archived: None,
            author: None,
            chats: None,
            subs: None,
            creation_date: None,
        };
    }
}

impl Chat {
    pub fn new(uuid: String) -> Chat {
        return Chat {
            uuid: Some(uuid),
            content: None,
            forum: None,
            creation_date: None,
            author: None,
            archived: None,
            answers: None,
            answer_to: None,
        };
    }
}

impl fmt::Display for Forum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut chats = vec![];
        if let Some(x) = &self.chats {
            for c in x {
                chats.push(&c.uuid);
            }
        }
        write!(f, "id: {uuid:?}, name: {name:?}, desc: {desc:?}, archived: {archived:?},  author: {author:?}, chats: {chat:?}, subs: {subs:?} creation_date: {creation_date:?}",
            uuid=self.uuid,
            name=self.name,
            desc=self.description,
            archived=self.archived,
            author=self.author,
            chat=chats,
            subs=self.subs,
            creation_date=self.creation_date,
        )
    }
}

impl fmt::Display for Chat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let forum = match &self.forum {
            Some(x) => Some(&x.uuid),
            None => None,
        };
        let author = match &self.author {
            Some(x) => Some(&x.uuid),
            None => None,
        };
        let mut answers = vec![];
        if let Some(ans) = &self.answers {
            for a in ans {
                if let Some(x) = &a.uuid {
                    answers.push(x);
                }
            }
        }
        let answer_to = match &self.answer_to {
            Some(x) => Some(&x.uuid),
            None => None,
        };
        write!(
            f,
            "id: {uuid:?}, content: {content:?}, forum: {forum:?}, creation_date: {date:?}, author: {author:?}, archived={archived:?}, answers: {answers:?}, answer_to: {answer_to:?}",
            uuid = self.uuid,
            content = self.content,
            forum = forum,
            date = self.creation_date,
            author = author,
            archived=self.archived,
            answers=answers,
            answer_to=answer_to,
        )
    }
}
