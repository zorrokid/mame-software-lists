CREATE TABLE rom (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    size INTEGER NOT NULL,
    crc TEXT NOT NULL,
    sha1 TEXT NOT NULL,
    machine_id INTEGER NOT NULL,
    FOREIGN KEY (machine_id) REFERENCES machine (id)
)