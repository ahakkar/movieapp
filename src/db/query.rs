//use diesel::debug_query;
//use diesel::sqlite::Sqlite;
use diesel::SqliteConnection;
use diesel::prelude::*;
use crate::db::model::*;
use crate::db::schema::*;

// Select all works from database
pub fn select_all_works(conn: &mut SqliteConnection) -> 
    Vec<(Work, Option<String>)>
{
    let query = work::table
        .left_join(work_type::table)
        .select((Work::as_select(), work_type::name.nullable()));


    // println!("{}", debug_query::<Sqlite, _>(&query));

    query
        .load::<(Work, Option<String>)>(conn)                 
        .expect("Error loading works")
}