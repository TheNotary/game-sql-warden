# The Window Wizard’s Trial — Instructions

You must create a SQL VIEW named **solution** that solves the following challenge.

You are given two tables:

### employees
| column        | type    | description                                |
|---------------|---------|--------------------------------------------|
| id            | integer | primary key                                |
| department    | text    | department name                            |
| employee_name | text    | the employee’s name                        |
| salary        | integer | the employee’s salary                      |

### projects
| column        | type    | description                                |
|---------------|---------|--------------------------------------------|
| id            | integer | primary key                                |
| employee_id   | integer | foreign key referencing employees(id)      |
| project_name  | text    | the name of a project an employee worked on|

---

## Your Task

Create a VIEW named **solution** that returns:

- Each **employee_name**
- Their **department**
- Their **salary**
- The **number of projects** they have worked on
- Their **rank within their department**, ordered by salary (highest salary gets rank 1)

Your VIEW must:
- Use **window functions**
- Use **partitioning**
- Use **ranking**, either `RANK()` or `DENSE_RANK()`
- Correctly count projects even if an employee has zero
- Group/aggregate without losing the ability to window-rank employees

The output schema must be:

| column          | description                                      |
|-----------------|--------------------------------------------------|
| employee_name   | text                                             |
| department      | text                                             |
| salary          | integer                                          |
| project_count   | integer                                          |
| salary_rank     | integer (1 = highest salary in department)       |

Your submission must create a VIEW named **solution** exactly.

Good luck, traveler. May your partitions be ever aligned.
