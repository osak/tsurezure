table! {
    posts {
        id -> Integer,
        body -> Text,
        posted_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}