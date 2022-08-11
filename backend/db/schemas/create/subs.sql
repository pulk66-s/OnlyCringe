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
