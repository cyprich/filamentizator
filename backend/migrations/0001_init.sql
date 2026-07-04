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

create table color (
    id serial primary key, 
    name varchar(64) not null, 
    hex char(6) not null, 
    unique(name, hex)
);

create table filament (
    id serial primary key, 
    vendor_id integer not null references vendor(id), 
    material_id integer not null references material(id), 
    name varchar(255) not null, 
    temp_min integer not null, 
    temp_max integer, 
    temp_bed_min integer not null, 
    temp_bed_max integer, 
    price real not null, 
    original_weight integer not null default 1000, 
    net_weight integer not null, 
    spool_weight integer not null,
    date_created timestamptz not null default now(), 
    date_updated timestamptz not null default now()
);

create table filament_color (
    id serial primary key,
    filament_id integer not null references filament(id), 
    color_id integer not null references color(id), 
    position integer,
    unique (filament_id, color_id, position)
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
