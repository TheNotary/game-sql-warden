SELECT
    sm.cube_id,
    sm.monster_id,
    CASE
        WHEN (sm.cube_id, sm.monster_id) IN (
            (1, 2),  -- Granite Fiend
            (2, 5),  -- Voltage Golem
            (3, 6)   -- Translucent Horror
        )
        THEN 1 ELSE 0
    END AS is_correct
FROM strongest_monsters sm
ORDER BY sm.cube_id;

