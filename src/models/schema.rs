// @generated automatically by Diesel CLI.

diesel::table! {
    events (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
        location -> Text,
    }
}
