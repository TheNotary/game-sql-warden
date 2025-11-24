-- Test for Joinocalypse Now
-- This query should return PASS or FAIL

WITH expected AS (
    SELECT
        e.name AS employee_name,
        d.name AS department_name,
        p.name AS project_name
    FROM employees e
    JOIN departments d ON d.id = e.department_id
    JOIN assignments a ON a.employee_id = e.id
    JOIN projects p ON p.id = a.project_id
)
SELECT
    CASE
        WHEN (
            SELECT COUNT(*) FROM (
                SELECT * FROM expected
                EXCEPT
                SELECT * FROM solution
            )
        ) = 0
        AND (
            SELECT COUNT(*) FROM (
                SELECT * FROM solution
                EXCEPT
                SELECT * FROM expected
            )
        ) = 0
        THEN 'PASS'
        ELSE 'FAIL'
    END AS result;
