CREATE TABLE roms (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL,
    size INTEGER NOT NULL,
    crc TEXT NOT NULL,
    sha1 TEXT NOT NULL
)