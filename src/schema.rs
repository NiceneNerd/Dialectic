table! {
    comments (id) {
        id -> Unsigned<Bigint>,
        name -> Varchar,
        #[sql_name = "comment"]
        body -> Text,
        upvotes -> Integer,
        date_posted -> Timestamp,
    }
}
