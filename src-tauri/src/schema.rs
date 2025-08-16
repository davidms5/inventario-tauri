// @generated automatically by Diesel CLI.

diesel::table! {
    combo_items (combo_id, product_id) {
        combo_id -> Integer,
        product_id -> Integer,
        cantidad -> Integer,
    }
}

diesel::table! {
    combos (id) {
        id -> Integer,
        nombre -> Text,
        descripcion -> Nullable<Text>,
        price -> Float,
        enabled -> Bool,
    }
}

diesel::table! {
    intentos_fallidos (username, fecha) {
        username -> Text,
        intentos -> Integer,
        fecha -> Text,
    }
}

diesel::table! {
    payments (id) {
        id -> Nullable<Integer>,
        sale_id -> Integer,
        monto -> Float,
        forma_pago -> Text,
        referencia -> Nullable<Text>,
    }
}

diesel::table! {
    products (id) {
        id -> Integer,
        nombre -> Text,
        sku -> Nullable<Text>,
        descripcion -> Nullable<Text>,
        price -> Float,
        quantity -> Integer,
        category -> Nullable<Text>,
        created_at -> Nullable<Text>,
        updated_at -> Nullable<Text>,
        enabled -> Bool,
    }
}

diesel::table! {
    sale_items (id) {
        id -> Integer,
        sale_id -> Integer,
        product_id -> Nullable<Integer>,
        combo_id -> Nullable<Integer>,
        cantidad -> Integer,
        precio_unitario -> Float,
        costo_unitario -> Float,
    }
}

diesel::table! {
    sales (id) {
        id -> Integer,
        user_id -> Integer,
        fecha -> Text,
        total -> Float,
        forma_pago -> Text,
        estado -> Text,
        deleted_at -> Nullable<Text>,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
        password_hash -> Text,
        created_at -> Nullable<Timestamp>,
        rol -> Nullable<Text>,
        enabled_add_products -> Bool,
    }
}

diesel::joinable!(combo_items -> combos (combo_id));
diesel::joinable!(combo_items -> products (product_id));
diesel::joinable!(payments -> sales (sale_id));
diesel::joinable!(sale_items -> combos (combo_id));
diesel::joinable!(sale_items -> products (product_id));
diesel::joinable!(sale_items -> sales (sale_id));
diesel::joinable!(sales -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    combo_items,
    combos,
    intentos_fallidos,
    payments,
    products,
    sale_items,
    sales,
    users,
);
