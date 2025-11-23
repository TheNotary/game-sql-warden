DROP TABLE IF EXISTS players;

CREATE TABLE players (
    id INTEGER PRIMARY KEY,
    team TEXT NOT NULL,
    name TEXT NOT NULL,
    score INTEGER NOT NULL
);

INSERT INTO players (team, name, score) VALUES
  ('Red', 'Auron', 95),
  ('Red', 'Selka', 88),
  ('Red', 'Brant', 92),
  ('Red', 'Tal', 70),
  ('Blue', 'Nira', 99),
  ('Blue', 'Kade', 85),
  ('Blue', 'Milo', 85),
  ('Green', 'Pox', 10);
