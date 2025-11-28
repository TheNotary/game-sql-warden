SELECT
    CASE
        WHEN COUNT(*) = SUM(
            CASE
                WHEN (sm.cube_id, sm.monster_id) IN (
                    (1, 2),  -- Granite Fiend
                    (2, 5),  -- Voltage Golem
                    (3, 6)   -- Translucent Horror
                ) THEN 1 ELSE 0
            END
        )
        THEN 'PASS'
        ELSE 'FAILED'
    END AS test_result
FROM solution sm;
