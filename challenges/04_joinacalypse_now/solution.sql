CREATE VIEW solution AS
SELECT
       employees.name AS employee_name,
       departments.name AS department_name,
       projects.name AS project_name
  FROM employees
  LEFT JOIN departments ON employees.department_id = departments.id
  JOIN assignments ON employees.id = assignments.employee_id
  LEFT JOIN projects ON assignments.project_id = projects.id;
