-- Add migration script here
CREATE TABLE scores (
    name VARCHAR PRIMARY KEY,
    flags INTEGER NOT NULL DEFAULT 0,
    updated INTEGER NOT NULL DEFAULT current_timestamp
);
CREATE TABLE flags (
    id INTEGER PRIMARY KEY,
    flag VARCHAR NOT NULL
);
INSERT INTO flags (id, flag)
VALUES
    (1, "FLAG-SecureSHip"),
    (2, "FLAG-ImNotTheCaptainYet"),
    (3, "FLAG-UnSecretEnArgument"),
    (4, "FLAG-SSHPortForwardingSavesLives"),
    (5, "FLAG-EscapeSequences"),
    (6, "FLAG-InsecureDirectObjectReferences"),
    (7, "FLAG-LookForSourceCode"),
    (8, "FLAG-ThereOnceWasAShipThatPutToSea"),
    (9, "FLAG-DumpingTheWholeDatabaseIsAnOption"),
    (10, "FLAG-LotsOfTreasuresAndRiches");
