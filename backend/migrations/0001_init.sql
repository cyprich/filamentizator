-- Add migration script here

---------- CREATE TABLES ----------
create table vendor (
    id serial primary key,
    name varchar(255) not null unique,
);

create table material (
    id serial primary key,
    name varchar(64) not null unique,
);

create table filament (
    id serial primary key, 
    vendor_id integer not null, 
    material_id integer not null, 
    name varchar(255) not null, 
    temp_min integer not null, 
    temp_max integer, 
    temp_bed_min integer not null, 
    temp_bed_max integer, 
    price real not null, 
    date_created timestamptz not null default now(), 
    date_updated timestamptz not null default now(), 
    foreign key (vendor_id) references vendor(id), 
    foreign key (material_id) references material(id)
);

---------- CREATE FUNCTIONS ----------
create or replace function set_date_edited_function()
returns trigger as $$
    begin
        new.date_updated := now();
        return new;
    end;
$$ language plpgsql;

---------- CREATE TRIGGERS ----------
create trigger vendor_date_edited_trigger
    before update on vendor
    for each row
    execute function set_date_edited_function();

create trigger material_date_edited_trigger
    before update on material
    for each row
    execute function set_date_edited_function();

create trigger filament_date_edited_trigger
    before update on filament
    for each row
    execute function set_date_edited_function();
