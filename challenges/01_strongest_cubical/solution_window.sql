CREATE VIEW strongest_monsters AS
WITH cte_thing AS (SELECT cubes.id AS cube_id,
       cubes.name AS cube_name,
       monsters.id AS monster_id,
       monsters.name AS monster_name,
       monsters.hp AS hp,
       RANK() OVER (PARTITION BY cubes.id ORDER BY monsters.hp DESC) AS monster_ranks
  FROM monsters
  JOIN cubes
    ON cubes.id = monsters.cube_id)

SELECT cube_id,
       cube_name,
       monster_id,
       monster_name,
       hp
  FROM cte_thing
 WHERE monster_ranks = 1;
