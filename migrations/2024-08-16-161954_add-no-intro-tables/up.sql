CREATE TABLE dat_file (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    version TEXT NOT NULL,
    author TEXT NOT NULL,
    homepage TEXT NOT NULL,
    url TEXT NOT NULL,
    system_id INTEGER NOT NULL,
    FOREIGN KEY (system_id) REFERENCES systems (id)
);

CREATE TABLE game (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    dat_file_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    FOREIGN KEY (dat_file_id) REFERENCES dat_file (id)
);

CREATE TABLE game_rom (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    game_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    size INTEGER NOT NULL,
    crc TEXT NOT NULL,
    md5 TEXT NOT NULL,
    sha1 TEXT NOT NULL,
    sha256 TEXT,
    status TEXT,
    serial TEXT,
    header TEXT,
    FOREIGN KEY (game_id) REFERENCES game (id)
);