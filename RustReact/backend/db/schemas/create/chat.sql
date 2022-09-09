use OnlyCringes;

drop table if exists Chat;
create table Chat (
    uuid UUID primary key not null unique default UUID(),
    author_id UUID not null,
    content text not null,
    forum_id UUID not null,
    creation_date datetime not null default now(),
    archived bool not null default false,
    answer_to UUID default null,
    foreign key (answer_to) references Chat(uuid) on delete cascade,
    foreign key (forum_id) references Forum(uuid) on delete cascade,
    foreign key (author_id) references User(uuid) on delete cascade
);
