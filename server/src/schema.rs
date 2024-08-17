// @generated automatically by Diesel CLI.

diesel::table! {
    clients (client_user_id) {
        client_user_id -> Text,
        access_token -> Nullable<Text>,
    }
}
