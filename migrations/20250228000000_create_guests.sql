-- Guests: uuid + first_name / last_name as JSON with "value" and "updated_at"
CREATE TABLE IF NOT EXISTS guests (
    id TEXT PRIMARY KEY NOT NULL,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL
);
