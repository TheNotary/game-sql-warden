CREATE TABLE cleared_levels (
    id INTEGER PRIMARY KEY,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE player (
    id INTEGER PRIMARY KEY,
    x INTEGER,
    y INTEGER
);

INSERT INTO player (id, x, y)
     VALUES (1, 5, 12);
