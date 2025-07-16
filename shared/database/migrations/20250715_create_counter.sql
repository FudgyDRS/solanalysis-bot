create table if not exists counters (
  id serial primary key,
  value bigint not null default 0
);

insert into counters (value)
select 0 where not exists (select 1 from counters);
