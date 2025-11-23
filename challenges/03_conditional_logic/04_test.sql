-- This test checks whether the user's VIEW named `solution`
-- correctly classifies heroes according to the challenge rules.

WITH expected AS (
    SELECT
        name,
        CASE
            WHEN health > 70 THEN 'strong'
            WHEN health BETWEEN 31 AND 70 THEN 'wounded'
            ELSE 'critical'
        END AS status
    FROM heroes
),
check_result AS (
    SELECT
        CASE
            WHEN (
                SELECT COUNT(*) FROM (
                    SELECT e.name, e.status
                    FROM expected e
                    EXCEPT
                    SELECT s.name, s.status
                    FROM solution s
                )
            ) = 0
            AND (
                SELECT COUNT(*) FROM (
                    SELECT s.name, s.status
                    FROM solution s
                    EXCEPT
                    SELECT e.name, e.status
                    FROM expected e
                )
            ) = 0
            THEN 'PASS'
            ELSE 'FAIL'
        END AS result
)
SELECT result FROM check_result;

