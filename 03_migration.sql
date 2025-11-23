-- Create cubes
CREATE TABLE cubes (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL
);

-- Create monsters
CREATE TABLE monsters (
    id INTEGER PRIMARY KEY,
    cube_id INTEGER,
    name TEXT NOT NULL,
    hp INTEGER,
    FOREIGN KEY (cube_id) REFERENCES cubes(id)
);

-- Seed cubes
INSERT INTO cubes (id, name) VALUES
    (1, 'Obsidian Alcove'),
    (2, 'Copper Prism'),
    (3, 'Glass Labyrinth');

-- Seed monsters
INSERT INTO monsters (id, cube_id, name, hp) VALUES
    (1, 1, 'Wraithling', 22),
    (2, 1, 'Granite Fiend', 80),
    (3, 1, 'Mold Imp', 35),
    (4, 2, 'Static Sprite', 15),
    (5, 2, 'Voltage Golem', 95),
    (6, 3, 'Translucent Horror', 60),
    (7, 3, 'Spectral Rabbit', 5);

