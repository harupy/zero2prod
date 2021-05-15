-- Add migration script here
CREATE TABLE subscriptiosn(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    email TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    -- timestamptz is a time-zone aware date and time type
    subscribed_at timestamptz NOT NULL
)
