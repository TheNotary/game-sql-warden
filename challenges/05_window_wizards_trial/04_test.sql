-- This test checks:
-- 1. Correct project counts
-- 2. Correct salary ranking per department
-- 3. Correct schema and values in VIEW "solution"

WITH expected AS (
    SELECT
        e.employee_name,
        e.department,
        e.salary,
        COUNT(p.id) AS project_count,
        DENSE_RANK() OVER (
            PARTITION BY e.department
            ORDER BY e.salary DESC
        ) AS salary_rank
    FROM employees e
    LEFT JOIN projects p ON e.id = p.employee_id
    GROUP BY e.id
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
