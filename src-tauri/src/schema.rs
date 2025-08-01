// @generated automatically by Diesel CLI.

diesel::table! {
    products (id) {
        id -> Nullable<Integer>,
        nombre -> Text,
        sku -> Nullable<Text>,
        descripcion -> Nullable<Text>,
        price -> Float,
        quantity -> Integer,
        category -> Nullable<Text>,
        created_at -> Nullable<Text>,
        updated_at -> Nullable<Text>,
    }
}

diesel::table! {
    users (id) {
        id -> Nullable<Integer>,
        username -> Text,
        password_hash -> Text,
        created_at -> Nullable<Timestamp>,
        rol -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    products,
    users,
);
