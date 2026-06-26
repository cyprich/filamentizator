-- Add migration script here
create table vendor (
    id serial primary key,
    name varchar(255) not null unique
)
