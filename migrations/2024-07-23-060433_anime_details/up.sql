-- Your SQL goes here
CREATE TABLE IF NOT EXISTS anime (
    id INT PRIMARY KEY,
    title VARCHAR(500) NOT NULL,
    description TEXT NOT NULL,
    mal_id INT NOT NULL,
    al_id INT NOT NULL,
    japanese_title VARCHAR(500),
    image VARCHAR(200) NOT NULL,
    category VARCHAR(50) NOT NULL,
    total_episodes INT NOT NULL,
    sub_or_dub VARCHAR(50) NOT NULL
);

CREATE TABLE IF NOT EXISTS anime_id (
      id SERIAL PRIMARY KEY,
      anime_name VARCHAR(500) UNIQUE NOT NULL
);

CREATE TABLE IF NOT EXISTS episodes (
    id VARCHAR(500) PRIMARY KEY,
    episode_no INT NOT NULL,
    title VARCHAR(500) NOT NULL,
    is_filler BOOLEAN NOT NULL,
    anime_id INT NOT NULL,
    FOREIGN KEY (anime_id) REFERENCES anime(id)
);
