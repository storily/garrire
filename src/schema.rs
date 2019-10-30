table! {
    nanowrimo_logins (id) {
        id -> Int4,
        token -> Text,
        expiry -> Int4,
        user_id -> Int4,
        created_at -> Timestamptz,
    }
}
