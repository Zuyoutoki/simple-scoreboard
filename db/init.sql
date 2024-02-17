-- Create tables
CREATE TABLE challenges (
    id      INTEGER PRIMARY KEY,
    flag    VARCHAR NOT NULL
);
CREATE TABLE submissions (
    id          INTEGER PRIMARY KEY,
    user        VARCHAR NOT NULL,
    challenge_id     INTEGER NOT NULL,
    timestamp   INTEGER NOT NULL DEFAULT current_timestamp,
    FOREIGN KEY(challenge_id) REFERENCES challenges(id)
);

-- Insert flags
INSERT INTO challenges (flag)
VALUES
    ('FLAG-First'),
    ('FLAG-Second'),
    ('FLAG-Third');
