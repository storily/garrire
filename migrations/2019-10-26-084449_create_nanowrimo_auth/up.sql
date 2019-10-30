CREATE TABLE nanowrimo_logins (
    id SERIAL PRIMARY KEY,
    token text NOT NULL,
    expiry integer NOT NULL,
    user_id integer NOT NULL,
    created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP
);
