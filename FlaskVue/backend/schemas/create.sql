drop database if exists OnlyCringe;
create database OnlyCringe;
use OnlyCringe;

create table User (
    uuid UUID primary key not null default UUID(),
    name text not null unique,
    password text not null,
    email text not null unique
);

create table Topics (
    uuid UUID primary key not null default UUID(),
    name text not null unique,
    author_uuid UUID not null,
    foreign key (author_uuid) references User(uuid) on delete cascade
);

create table Post (
    uuid UUID primary key not null default UUID(),
    title text not null,
    content text not null,
    author_uuid UUID not null,
    topic_uuid UUID not null,
    foreign key (author_uuid) references User(uuid) on delete cascade,
    foreign key (topic_uuid) references Topics(uuid) on delete cascade
);
