table! {
    notifications (id) {
        id -> Integer,
        user_id -> Integer,
        order_id -> Integer,
        account_id -> Integer,
        order_amount -> Float,
        was_succeed -> Bool,
        date -> Timestamp,
    }
}


allow_tables_to_appear_in_same_query!(
    notifications,
);
