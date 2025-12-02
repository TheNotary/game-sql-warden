DROP VIEW IF EXISTS solution;
CREATE VIEW solution AS
WITH project_count AS (
    SELECT employee_id, COUNT(*) AS total_projects
    FROM projects
    GROUP BY 1
)
SELECT employees.employee_name
    , employees.department
    , employees.salary
    , CASE
        WHEN project_count.total_projects IS NULL
            THEN 0
        ELSE project_count.total_projects
        END AS total_projects
    , RANK() OVER (PARTITION BY employees.department ORDER BY employees.salary DESC) AS department_rank
FROM employees
LEFT JOIN project_count
    ON employees.id = project_count.employee_id;
