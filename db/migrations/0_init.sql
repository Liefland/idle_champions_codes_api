create table if not exists sources (
    id serial not null,

    name varchar(255) not null,
    url varchar(2048) not null,

    primary key (id),
    unique (name, url)
);

create table if not exists api_keys (
    id serial not null,

    api_key varchar(255) not null,
    source_id int not null,

    expired boolean default false,

    primary key (id),
    foreign key (source_id) references sources(id)
);
create index if not exists api_keys_idx on api_keys (api_key);

create table if not exists codes (
    id serial not null,

    code varchar(64) not null,
    expires_at timestamp not null,

    submitter_id int not null, -- a reference to the person who submitted the code
    creator_id int not null, -- a reference to the person, streamer, service who created the code
    lister_id int not null, -- a reference to the service that listed the code on this site

    primary key (id),
    foreign key (lister_id) references sources(id),
    foreign key (submitter_id) references sources(id),
    foreign key (creator_id) references sources(id),
    unique (expires_at, code)
);
