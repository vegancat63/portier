// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Int4,
        title -> Varchar,
        description -> Nullable<Text>,
        account_type -> Nullable<Varchar>,
        protocol -> Nullable<Varchar>,
        login -> Nullable<Varchar>,
        password -> Nullable<Varchar>,
        host -> Nullable<Varchar>,
        port -> Nullable<Varchar>,
        url -> Nullable<Varchar>,
    }
}
