create database if not exists OnlyCringes;
CREATE USER 'OCUser'@'%' IDENTIFIED BY 'ççOCUs3r_p@ssw0rd__';
GRANT ALL PRIVILEGES ON OnlyCringes.* TO 'OCUser'@'%';
use OnlyCringes;

create table User (
    uuid UUID primary key not null default UUID(),
    name text not null unique,
    description text not null default "",
    password text not null,
    email text not null unique,
    archived bool not null default false,
    creation_date datetime not null default now(),
    role enum("ADMIN", "USER") not null default "USER"
);
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
use OnlyCringes

create table Subs (
    uuid UUID primary key not null default UUID(),
    uid UUID not null,
    fid UUID not null,
    foreign key (uid) references User(uuid) on delete cascade,
    foreign key (fid) references Forum(uuid) on delete cascade
);

ALTER TABLE OnlyCringes.Subs
    ADD CONSTRAINT Subs UNIQUE(uid, fid);
use OnlyCringes;

create table Friend (
    uuid UUID primary key not null default UUID(),
    uid UUID not null,
    fid UUID not null,
    status enum("ACCEPTED", "DECLINED", "ASKING") not null default "ASKING",
    creation_date datetime not null default now(),
    foreign key (uid) references User(uuid) on delete cascade,
    foreign key (fid) references User(uuid) on delete cascade
);

ALTER TABLE OnlyCringes.Friend
    ADD CONSTRAINT Friend UNIQUE(uid, fid);
