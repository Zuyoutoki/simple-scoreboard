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
    ('FLAG-SecureSHip'),
    ('FLAG-ImNotTheCaptainYet'),
    ('FLAG-UnSecretEnArgument'),
    ('FLAG-SSHPortForwardingSavesLives'),
    ('FLAG-EscapeSequences'),
    ('FLAG-InsecureDirectObjectReferences'),
    ('FLAG-LookForSourceCode'),
    ('FLAG-ThereOnceWasAShipThatPutToSea'),
    ('FLAG-DumpingTheWholeDatabaseIsAnOption'),
    ('FLAG-LotsOfTreasuresAndRiches');
