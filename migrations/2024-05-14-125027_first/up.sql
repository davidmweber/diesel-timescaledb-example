create extension if not exists timescaledb;

create table speedy (
    time timestamptz not null,
    serial_number text not null,
    c00 float4,
    c01 float4,
    c02 float4,
    c03 float4,
    c04 float4,
    c05 float4,
    c06 float4,
    c07 float4,
    c08 float4,
    c09 float4,
    c10 float4,
    c11 float4,
    c12 float4,
    c13 float4,
    c14 float4,
    c15 float4,
    c16 float4,
    c17 float4,
    c18 float4,
    c19 float4,
    c20 float4,
    c21 float4,
    c22 float4,
    c23 float4,
    c24 float4,
    c25 float4,
    c26 float4,
    c27 float4,
    c28 float4,
    c29 float4,
    c30 float4,
    c31 float4,
    c32 float4,
    c33 float4,
    c34 float4,
    c35 float4,
    c36 float4,
    c37 float4,
    c38 float4,
    c39 float4,
    c40 float4[] not null,
    primary key (time, serial_number)
);

select * from create_hypertable('speedy', by_range('time'));
