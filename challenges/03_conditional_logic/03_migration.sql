-- Create base table for the challenge
CREATE TABLE heroes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    health INTEGER NOT NULL
);

-- Seed data
INSERT INTO heroes (name, health) VALUES
    ('Astra', 95),
    ('Bram', 72),
    ('Cinder', 70),
    ('Dorian', 45),
    ('Eve', 30),
    ('Falk', 10);

