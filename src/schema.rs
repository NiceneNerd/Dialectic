table! {
    comments (id) {
        id -> Unsigned<Bigint>,
        parent_id -> Nullable<Unsigned<Bigint>>,
        name -> Varchar,
        #[sql_name = "comment"]
        body -> Text,
        upvotes -> Integer,
        date_posted -> Timestamp,
    }
}
