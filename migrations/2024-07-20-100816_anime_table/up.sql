-- Your SQL goes here
CREATE TABLE IF NOT EXISTS anime (
    id INT PRIMARY KEY,
    title VARCHAR(500) NOT NULL,
    description TEXT NOT NULL,
    mal_id INT NOT NULL,
    al_id INT NOT NULL,
    japanese_title VARCHAR(500),
    image VARCHAR(200) NOT NULL,
    type VARCHAR(50) NOT NULL,
    sub_or_dub VARCHAR(50) NOT NULL
);
