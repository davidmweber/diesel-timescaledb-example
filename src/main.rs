use std::time::Instant;
use diesel::internal::derives::multiconnection::chrono;
use diesel::{insert_into, PgConnection};
use diesel::internal::derives::multiconnection::chrono::NaiveDateTime;
use dotenvy::dotenv;
use diesel::prelude::*;
use rand::Rng;

pub mod schema;

use crate::schema::speedy::dsl::*;

fn foo(i: i32) -> (diesel::dsl::Eq<time, NaiveDateTime>, diesel::dsl::Eq<serial_number, String>, diesel::dsl::Eq<c00, Option<f32>>, diesel::dsl::Eq<c01, Option<f32>>, diesel::dsl::Eq<c02, Option<f32>>, diesel::dsl::Eq<c03, Option<f32>>, diesel::dsl::Eq<c04, Option<f32>>, diesel::dsl::Eq<c05, Option<f32>>, diesel::dsl::Eq<c06, Option<f32>>, diesel::dsl::Eq<c07, Option<f32>>, diesel::dsl::Eq<c08, Option<f32>>, diesel::dsl::Eq<c09, Option<f32>>, diesel::dsl::Eq<c10, Option<f32>>, diesel::dsl::Eq<c11, Option<f32>>, diesel::dsl::Eq<c12, Option<f32>>, diesel::dsl::Eq<c13, Option<f32>>, diesel::dsl::Eq<c14, Option<f32>>, diesel::dsl::Eq<c15, Option<f32>>, diesel::dsl::Eq<c16, Option<f32>>, diesel::dsl::Eq<c17, Option<f32>>, diesel::dsl::Eq<c18, Option<f32>>, diesel::dsl::Eq<c19, Option<f32>>, diesel::dsl::Eq<c20, Option<f32>>, diesel::dsl::Eq<c21, Option<f32>>, diesel::dsl::Eq<c22, Option<f32>>, diesel::dsl::Eq<c23, Option<f32>>, diesel::dsl::Eq<c24, Option<f32>>, diesel::dsl::Eq<c25, Option<f32>>, diesel::dsl::Eq<c26, Option<f32>>, diesel::dsl::Eq<c27, Option<f32>>, diesel::dsl::Eq<c28, Option<f32>>, diesel::dsl::Eq<c29, Option<f32>>, diesel::dsl::Eq<c30, Option<f32>>, diesel::dsl::Eq<c31, Option<f32>>, diesel::dsl::Eq<c32, Option<f32>>, diesel::dsl::Eq<c33, Option<f32>>, diesel::dsl::Eq<c34, Option<f32>>, diesel::dsl::Eq<c35, Option<f32>>, diesel::dsl::Eq<c36, Option<f32>>, diesel::dsl::Eq<c37, Option<f32>>, diesel::dsl::Eq<c38, Option<f32>>, diesel::dsl::Eq<c39, Option<f32>>, diesel::dsl::Eq<c40, Vec<f32>>) {
    let mut rng = rand::thread_rng();
    (time.eq(chrono::Utc::now().naive_utc()),
     serial_number.eq(format!("ABS-{:0>4}", i)),
     c00.eq(Some(rng.gen::<f32>())),
     c01.eq(Some(rng.gen::<f32>())),
     c02.eq(Some(rng.gen::<f32>())),
     c03.eq(Some(rng.gen::<f32>())),
     c04.eq(Some(rng.gen::<f32>())),
     c05.eq(Some(rng.gen::<f32>())),
     c06.eq(Some(rng.gen::<f32>())),
     c07.eq(Some(rng.gen::<f32>())),
     c08.eq(Some(rng.gen::<f32>())),
     c09.eq(Some(rng.gen::<f32>())),
     c10.eq(Some(rng.gen::<f32>())),
     c11.eq(Some(rng.gen::<f32>())),
     c12.eq(Some(rng.gen::<f32>())),
     c13.eq(Some(rng.gen::<f32>())),
     c14.eq(Some(rng.gen::<f32>())),
     c15.eq(Some(rng.gen::<f32>())),
     c16.eq(Some(rng.gen::<f32>())),
     c17.eq(Some(rng.gen::<f32>())),
     c18.eq(Some(rng.gen::<f32>())),
     c19.eq(Some(rng.gen::<f32>())),
     c20.eq(Some(rng.gen::<f32>())),
     c21.eq(Some(rng.gen::<f32>())),
     c22.eq(Some(rng.gen::<f32>())),
     c23.eq(Some(rng.gen::<f32>())),
     c24.eq(Some(rng.gen::<f32>())),
     c25.eq(Some(rng.gen::<f32>())),
     c26.eq(Some(rng.gen::<f32>())),
     c27.eq(Some(rng.gen::<f32>())),
     c28.eq(Some(rng.gen::<f32>())),
     c29.eq(Some(rng.gen::<f32>())),
     c30.eq(Some(rng.gen::<f32>())),
     c31.eq(Some(rng.gen::<f32>())),
     c32.eq(Some(rng.gen::<f32>())),
     c33.eq(Some(rng.gen::<f32>())),
     c34.eq(Some(rng.gen::<f32>())),
     c35.eq(Some(rng.gen::<f32>())),
     c36.eq(Some(rng.gen::<f32>())),
     c37.eq(Some(rng.gen::<f32>())),
     c38.eq(Some(rng.gen::<f32>())),
     c39.eq(Some(rng.gen::<f32>())),
     c40.eq(vec![rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()])
    )
}

fn main() {
    dotenv().ok();
    let url = std::env::var("DATABASE_URL").unwrap();
    let mut conn = PgConnection::establish(&url).unwrap();
    let now = Instant::now();
    let mut count = 0;
    for _t in 0..1000 {
        let mut b = vec![];
        for i in 0..1000 {
            b.push(foo(i));
            count += 1;
        }
        insert_into(speedy).values(&b)
            .on_conflict_do_nothing()
            .execute(&mut conn).unwrap();
        let elapsed = now.elapsed();
        println!("Inserted {} rows in {} ms at {} rows per second", count, elapsed.as_millis(), count as f64 / elapsed.as_secs_f64());
    }
    let elapsed = now.elapsed();
    println!("Final: Inserted {} rows in {} ms at {} rows per second", count, elapsed.as_millis(), count as f64 / elapsed.as_secs_f64());
}
