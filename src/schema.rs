// @generated automatically by Diesel CLI.

diesel::table! {
    cities (id) {
        id -> Nullable<Integer>,
        name -> Text,
        short_name -> Nullable<Text>,
    }
}
