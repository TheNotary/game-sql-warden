# Challenge: Joinocalypse Now

Your task is to demonstrate mastery of **multi-way joins** and join-driven problem solving.

You must create a **VIEW named `solution`** that returns the following columns:

- `employee_name`
- `department_name`
- `project_name`

### Goal

Produce a list of **all employees**, the departments they belong to, and the projects they are assigned to.  
Only employees **who actually have at least one project assignment** should appear.

### Schema Summary

You will be given the following tables:

- **employees(id, name, department_id)**
- **departments(id, name)**
- **projects(id, name)**
- **assignments(employee_id, project_id)**

### Requirements

1. Your `solution` view must:
   - Join all relevant tables using correct join conditions.
   - Only include rows where an employee has a matching project assignment.
   - Return the columns in the exact order:
     ```
     employee_name, department_name, project_name
     ```
2. Use **INNER JOINs** or equivalent logic to ensure only valid triples appear.
3. Your submission must be a single `CREATE VIEW solution AS ...` statement.

Good luck, brave query-runner.
