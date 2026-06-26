-- Add migration script here

---------- CREATE TABLES ----------
create table vendor (
    id serial primary key,
    name varchar(255) not null unique,
    date_created timestamptz not null default now(),
    date_edited timestamptz not null
);

---------- CREATE FUNCTIONS ----------
create or replace function set_vendor_date_edited_function()
returns trigger as $$
    begin
        new.date_edited := now();
        return new;
    end;
$$ language plpgsql;

---------- CREATE TRIGGERS ----------
create trigger vendor_date_edited_trigger
    before update on vendor
    for each row
    execute function set_vendor_date_edited_function();
