CREATE TABLE machine (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    description TEXT NOT NULL,
    year INTEGER NOT NULL,
    publisher TEXT NOT NULL,
    software_list_id INTEGER NOT NULL,
    FOREIGN KEY (software_list_id) REFERENCES software_list (id)
)
