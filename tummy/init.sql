CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    real_name TEXT NOT NULL,
    display_name TEXT NOT NULL,
    email TEXT NOT NULL,
    deleted BOOLEAN NOT NULL DEFAULT FALSE,
    is_bot BOOLEAN NOT NULL DEFAULT FALSE,
    IMAGE_url TEXT
);


CREATE TABLE IF NOT EXISTS channels (
    name TEXT PRIMARY KEY,
    topic TEXT,
    purpose TEXT,
);


CREATE TABLE IF NOT EXISTS messages (
    channel_name TEXT NOT NULL,
    user TEXT NOT NULL,
    ts TEXT NOT NULL,
    text TEXT NOT NULL,
    thread_ts TEXT,
    parent_user_id TEXT,
    PRIMARY KEY (user, ts),
    FOREIGN KEY (user) REFERENCES users(id),
    FOREIGN KEY (channel_name) REFERENCES channels(name)
);