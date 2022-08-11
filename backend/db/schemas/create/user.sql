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
