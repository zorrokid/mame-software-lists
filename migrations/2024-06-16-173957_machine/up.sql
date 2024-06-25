CREATE TABLE machines (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    description TEXT NOT NULL,
    year INTEGER,
    publisher TEXT NOT NULL,
    software_list_id INTEGER NOT NULL,
    FOREIGN KEY (software_list_id) REFERENCES software_lists (id)
)
