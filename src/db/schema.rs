// @generated automatically by Diesel CLI.

diesel::table! {
    artwork (id) {
        id -> Nullable<Integer>,
        work_id -> Integer,
        file_path -> Text,
        image_type -> Integer,
        image_description -> Nullable<Text>,
    }
}

diesel::table! {
    artwork_type (id) {
        id -> Nullable<Integer>,
        name -> Text,
    }
}

diesel::table! {
    genre (id) {
        id -> Nullable<Integer>,
        name -> Text,
    }
}

diesel::table! {
    person (id) {
        id -> Nullable<Integer>,
        prefix -> Nullable<Text>,
        first_name -> Text,
        middle_names -> Nullable<Text>,
        last_name -> Text,
        suffix -> Nullable<Text>,
        date_of_birth -> Nullable<Text>,
        date_of_death -> Nullable<Text>,
        biography -> Nullable<Text>,
        nationality -> Nullable<Text>,
    }
}

diesel::table! {
    rating (id) {
        id -> Nullable<Integer>,
        work_id -> Integer,
        rating_value -> Integer,
        rating_source -> Nullable<Text>,
        rating_date -> Nullable<Text>,
    }
}

diesel::table! {
    review (id) {
        id -> Nullable<Integer>,
        work_id -> Integer,
        review_text -> Nullable<Text>,
        reviewer_name -> Nullable<Text>,
        review_date -> Nullable<Text>,
    }
}

diesel::table! {
    role (id) {
        id -> Nullable<Integer>,
        role_name -> Text,
    }
}

diesel::table! {
    work (id) {
        id -> Nullable<Integer>,
        title -> Text,
        release_date -> Nullable<Text>,
        #[sql_name = "type"]
        type_ -> Nullable<Text>,
        summary -> Nullable<Text>,
        runtime -> Nullable<Integer>,
        language -> Nullable<Text>,
        network -> Nullable<Text>,
        status -> Nullable<Text>,
    }
}

diesel::table! {
    work_genre (rowid) {
        rowid -> Integer,
        work_id -> Integer,
        genre_id -> Integer,
    }
}

diesel::table! {
    work_person (id) {
        id -> Nullable<Integer>,
        work_id -> Integer,
        person_id -> Integer,
        role_id -> Integer,
        character_suffix -> Nullable<Text>,
        character_first_name -> Text,
        character_middle_names -> Nullable<Text>,
        character_last_name -> Text,
        character_prefix -> Nullable<Text>,
    }
}

diesel::joinable!(artwork -> artwork_type (image_type));
diesel::joinable!(artwork -> work (work_id));
diesel::joinable!(rating -> work (work_id));
diesel::joinable!(review -> work (work_id));
diesel::joinable!(work_genre -> genre (genre_id));
diesel::joinable!(work_genre -> work (work_id));
diesel::joinable!(work_person -> person (person_id));
diesel::joinable!(work_person -> role (role_id));
diesel::joinable!(work_person -> work (work_id));

diesel::allow_tables_to_appear_in_same_query!(
    artwork,
    artwork_type,
    genre,
    person,
    rating,
    review,
    role,
    work,
    work_genre,
    work_person,
);
