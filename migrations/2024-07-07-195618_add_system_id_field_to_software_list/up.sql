ALTER TABLE software_lists ADD COLUMN system_id INTEGER REFERENCES systems(id);