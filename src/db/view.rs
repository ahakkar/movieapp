diesel::table! {
    work_with_details (work_id) {
        work_id -> Integer,
        title -> Text,
        release_date -> Nullable<Text>,
        work_type -> Integer,
        summary -> Nullable<Text>,
        runtime -> Nullable<Integer>,
        language -> Nullable<Text>,
        network -> Nullable<Text>,
        status -> Nullable<Text>,
        work_type_name -> Nullable<Text>, 
        rating_value -> Nullable<Integer>,     
    }
}