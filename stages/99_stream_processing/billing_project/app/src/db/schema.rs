table! {
    accounts (id) {
        id -> Integer,
        user_id -> Integer,
        user_name -> Text,
        available -> Nullable<Float>,
        spent -> Nullable<Float>,
        create_date -> Timestamp,
        update_date -> Timestamp,
    }
}


allow_tables_to_appear_in_same_query!(
    accounts,
);
