table! {
    nanowrimo_logins (id) {
        id -> Int4,
        token -> Text,
        expiry -> Int4,
        user_id -> Int4,
        created_at -> Timestamptz,
    }
}

table! {
    users (id) {
        id -> Int4,
        discord_id -> BigInt,
        nick -> Nullable<Text>,
        nano_user -> Nullable<Text>,
        first_seen -> Timestamptz,
        last_seen -> Timestamptz,
        tz -> Text,
        updated -> Timestamptz,
    }
}
