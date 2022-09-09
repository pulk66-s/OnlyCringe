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
