CREATE VIEW strongest_monsters AS
WITH max_hp_monsters AS (
    SELECT cube_id, MAX(hp) AS max_hp
      FROM monsters
  GROUP BY cube_id
)
SELECT c.id       AS cube_id,
       c.name     AS cube_name,
       m.id       AS monster_id,
       m.name     AS monster_name,
       m.hp
FROM cubes c
JOIN max_hp_monsters x ON x.cube_id = c.id
JOIN monsters m ON m.cube_id = x.cube_id
               AND m.hp = x.max_hp;
