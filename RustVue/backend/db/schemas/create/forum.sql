use OnlyCringes;

drop table if exists Forum;
create table Forum (
    uuid UUID primary key not null unique default UUID(),
    name varchar(255) not null unique,
    description text not null default "",
    archived bool not null default false,
    author_id UUID not null,
    creation_date datetime not null default now(),
    foreign key (author_id) references User(uuid) on delete cascade
);
