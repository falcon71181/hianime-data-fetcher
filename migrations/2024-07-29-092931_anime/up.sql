-- Create the 'anime' table if it does not exist
CREATE TABLE IF NOT EXISTS anime (
    id              INT PRIMARY KEY,
    title           VARCHAR(500) NOT NULL,
    description     TEXT NOT NULL,
    mal_id          INT NOT NULL,
    al_id           INT NOT NULL,
    japanese_title  VARCHAR(500),
    synonyms        VARCHAR(500),
    image           VARCHAR(200) NOT NULL,
    category        VARCHAR(50) NOT NULL,
    rating          VARCHAR(50) NOT NULL,
    quality         VARCHAR(50) NOT NULL,
    duration        VARCHAR(50) NOT NULL,
    premiered       VARCHAR(100) NOT NULL,
    aired           VARCHAR(100) NOT NULL,
    status          VARCHAR(50) NOT NULL,
    mal_score       VARCHAR(50) NOT NULL,
    studios         TEXT NOT NULL,
    producers       TEXT NOT NULL,
    genres          TEXT NOT NULL,
    sub_episodes    INT NOT NULL,
    dub_episodes    INT NOT NULL,
    total_episodes  INT NOT NULL,
    sub_or_dub      VARCHAR(50) NOT NULL
);

-- Create the 'anime_id' table if it does not exist
CREATE TABLE IF NOT EXISTS anime_id (
    id             SERIAL PRIMARY KEY,
    anime_name     VARCHAR(500) UNIQUE NOT NULL
);

-- Create the 'episodes' table if it does not exist
CREATE TABLE IF NOT EXISTS episodes (
    id          VARCHAR(500) PRIMARY KEY,
    episode_no  INT NOT NULL,
    title       VARCHAR(500) NOT NULL,
    is_filler   BOOLEAN NOT NULL,
    anime_id    INT NOT NULL,
    FOREIGN KEY (anime_id) REFERENCES anime(id)
);

-- Create the 'staff' table if it does not exist
CREATE TABLE IF NOT EXISTS staff (
    mal_id      INT PRIMARY KEY,
    name        VARCHAR(255) NOT NULL,
    mal_url     VARCHAR(500) NOT NULL,
    image       JSONB NOT NULL,
    positions   TEXT[] NOT NULL
);

-- Create the 'anime_staff' table if it does not exist
CREATE TABLE IF NOT EXISTS anime_staff (
    anime_id    INT NOT NULL,
    staff_id    INT NOT NULL,
    positions   TEXT[] NOT NULL,
    PRIMARY KEY (anime_id, staff_id),
    FOREIGN KEY (anime_id) REFERENCES anime(id) ON DELETE CASCADE,
    FOREIGN KEY (staff_id) REFERENCES staff(mal_id) ON DELETE CASCADE
);

-- Create an index on the 'mal_id' column of the 'anime' table
CREATE INDEX IF NOT EXISTS idx_anime_mal_id ON anime (mal_id);

-- Create an index on the 'anime_id' column of the 'episodes' table
CREATE INDEX IF NOT EXISTS idx_episodes_anime_id ON episodes (anime_id);

-- Create indexes on the 'anime_id' and 'staff_id' columns of the 'anime_staff' table
CREATE INDEX IF NOT EXISTS idx_anime_staff_anime_id ON anime_staff (anime_id);
CREATE INDEX IF NOT EXISTS idx_anime_staff_staff_id ON anime_staff (staff_id);
