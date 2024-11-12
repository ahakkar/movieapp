use diesel::prelude::*;
use crate::db::view::*;

// Models for schema.rs
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::artwork)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Artwork {
    pub id: Option<i32>,
    pub work_id: i32,
    pub file_path: String,
    pub image_type: i32,
    pub image_description: Option<String>,
}


#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::artwork_type)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ArtworkType {
    pub id: Option<i32>,
    pub name: String,
}


#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::genre)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Genre {
    pub id: Option<i32>,
    pub name: String,
}


#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::person)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Person {
    pub id: Option<i32>,
    pub prefix: Option<String>,
    pub first_name: String,
    pub middle_names: Option<String>,
    pub last_name: String,
    pub suffix: Option<String>,
    pub date_of_birth: Option<String>,
    pub date_of_death: Option<String>,
    pub biography: Option<String>,
    pub nationality: Option<String>,
}


#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::rating)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Rating {
    pub id: Option<i32>,
    pub work_id: i32,
    pub rating_value: i32,
    pub rating_source: Option<String>,
    pub rating_date: Option<String>,
}


#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::review)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Review {
    pub id: Option<i32>,
    pub work_id: i32,
    pub review_text: Option<String>,
    pub reviewer_name: Option<String>,
    pub review_date: Option<String>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::role)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Role {
    pub id: Option<i32>,
    pub role_name: String,
}


#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::work)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Work {
    pub id: Option<i32>,
    pub title: String,
    pub release_date: Option<String>,
    pub work_type: i32,
    pub summary: Option<String>,
    pub runtime: Option<i32>,
    pub language: Option<String>,
    pub network: Option<String>,
    pub status: Option<String>,
}


#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::work_genre)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct WorkGenre {
    pub rowid: i32,
    pub work_id: i32,
    pub genre_id: i32,
}


#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::work_person)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct WorkPerson {
    pub id: Option<i32>,
    pub work_id: i32,
    pub person_id: i32,
    pub role_id: i32,
    pub character_suffix: Option<String>,
    pub character_first_name: String,
    pub character_middle_names: Option<String>,
    pub character_last_name: String,
    pub character_prefix: Option<String>,
}


#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::work_type)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct WorkType {
    pub id: Option<i32>,
    pub name: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::view::work_with_details)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct WorkWithDetails {
    pub work_id: i32,
    pub title: String,
    pub release_date: Option<String>,
    pub work_type: i32,
    pub summary: Option<String>,
    pub runtime: Option<i32>,
    pub language: Option<String>,
    pub network: Option<String>,
    pub status: Option<String>,
    pub work_type_name: Option<String>, 
    pub rating_value: Option<i32>,     
}