CREATE TABLE machines_roms (
    machine_id INTEGER NOT NULL,
    rom_id INTEGER NOT NULL,
    FOREIGN KEY (machine_id) REFERENCES machines (id),
    FOREIGN KEY (rom_id) REFERENCES roms (id),
    PRIMARY KEY (machine_id, rom_id)
)