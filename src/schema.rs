table! {
    comments (id) {
        id -> Integer,
        username -> Varchar,
        email -> Nullable<Varchar>,
        body -> Text,
        date_posted -> Timestamp,
    }
}
