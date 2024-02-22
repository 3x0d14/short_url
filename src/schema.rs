// @generated automatically by Diesel CLI.

diesel::table! {
    urls (origin) {
        origin -> Varchar,
        destination -> Varchar,
    }
}
