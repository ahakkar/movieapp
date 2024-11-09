use diesel::SqliteConnection;
use diesel::prelude::*;
use crate::db::model::*;
use crate::db::schema::work::dsl::*;

// Select all works from database
pub fn select_all_works(conn: &mut SqliteConnection) -> Vec<Work> {
    work
        .select(Work::as_select())
        .load(conn)                 
        .expect("Error loading works")
}