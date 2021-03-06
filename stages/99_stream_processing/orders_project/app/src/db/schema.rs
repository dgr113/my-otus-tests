table! {
    order_products (id) {
        id -> Integer,
        order_id -> Integer,
        price -> Float,
        quantity -> Integer,
        product_name -> Text,
        product_id -> Nullable<Integer>,
    }
}

table! {
    orders (id) {
        id -> Integer,
        create_date -> Timestamp,
        update_date -> Timestamp,
        customer_id -> Integer,
        status_code -> Integer,
    }
}

table! {
    order_unique_keys (id) {
        id -> Integer,
        order_id -> Integer,
        unique_key -> Text,
    }
}

joinable!(order_products -> orders (order_id));
joinable!(order_unique_keys -> orders (order_id));

allow_tables_to_appear_in_same_query!(
    order_products,
    orders,
);
