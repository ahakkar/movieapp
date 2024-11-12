use diesel::debug_query;
use diesel::sqlite::Sqlite;
use diesel::SqliteConnection;
use diesel::prelude::*;
use crate::db::model::*;
use crate::db::schema::*;

// Select all works from database
pub fn select_all_works(conn: &mut SqliteConnection) -> 
    Vec<WorkWithDetails>
{
    let query = work_with_details::table
        .select(WorkWithDetails::as_select())
        .order(work_with_details::release_date.desc());


    println!("{}", debug_query::<Sqlite, _>(&query));

    query
        .load::<WorkWithDetails>(conn)                 
        .expect("Error loading works")
}