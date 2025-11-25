SELECT cube_id, cube_name, monster_id, monster_name, hp


SELECT cube_id, MAX(hp) FROM monsters GROUP BY cube_id;






SELECT cube_id, name, hp
FROM monsters m
WHERE hp = (
    SELECT MAX(hp) 
    FROM monsters 
    WHERE cube_id = m.cube_id
);
