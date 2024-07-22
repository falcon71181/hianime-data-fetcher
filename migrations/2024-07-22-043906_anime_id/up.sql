-- Your SQL goes here
-- Your SQL goes here
CREATE TABLE IF NOT EXISTS anime_id (
      id SERIAL PRIMARY KEY,
      anime_name VARCHAR(500) UNIQUE NOT NULL
);
