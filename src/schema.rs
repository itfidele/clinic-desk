// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
        password -> Text,
        email -> Text,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}
