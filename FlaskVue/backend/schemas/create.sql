drop database if exists OnlyCringe;
create database OnlyCringe;
use OnlyCringe;

create table User (
    uuid UUID primary key not null default UUID(),
    name text not null unique,
    password text not null
);
