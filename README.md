# Diesel Timescale Example
A minimalist Diesel and Timescale example that I created primarily to evaluate write performance and 
interop.

## Setup and running
You just need a Rust environment and a working Docker to get running. Start TimescaleDB using Docker:
```bash
docker compose up -d
```
Note that it used use of port 5433 for Timescale as I have a local Postgres running on 5432. This will create
a database called `example` with a user name and password both seet to `test`. 

Install the [diesel cli](https://diesel.rs) and run the migrations:
```bash
cargo install diesel_cli --no-default-features --features postgres
diesel migration run
```
If the `diesel_cli` install fails because of a missing Postgres library, install those as for your platform.

Now run the application:
```bash
cargo run --release
```

## Implementation notes
Diesel requires a primary key but Timescale explicitly does not want a primary key in the table for
cases where you want multiple keys. The symptom is you cannot create a hypertable with a primary key
and Diesel requires a primary key for it's DSL to work. The solution is to declare a primary key as
in the example below.  In this case, we need a serial number and a time stamp as an index.
Create an explicit primary key across the two columns in the table definition, giving us a primary key
and a useful index while keeping Diesel and Timescale happy.

```sql
CREATE TABLE IF NOT EXISTS my_table (
    id SERIAL PRIMARY KEY,
    time TIMESTAMPTZ NOT NULL,
    data float4,
    primary key (id, time)                            
);
SELECT * from create_hypertable('my_table', by_range('time'));
SELECT * from add_dimension('my_table', by_hash('id', 4));  -- Or some such
```

All regular Diesel operations work as expected. The only thing to note is that you cannot use the DSL for the
TimescaleDB specific operations. Raw SQL queries should work fine for all Timescale queries.  The example provided 
creates tuples of rows to create a batch insert which is 20x faster than individual inserts. The tuple declaration is 
a bear but auto-complete in RustRover solved that.

