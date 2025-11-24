-- Employees table
CREATE TABLE employees (
    id INTEGER PRIMARY KEY,
    department TEXT NOT NULL,
    employee_name TEXT NOT NULL,
    salary INTEGER NOT NULL
);

-- Projects table
CREATE TABLE projects (
    id INTEGER PRIMARY KEY,
    employee_id INTEGER NOT NULL,
    project_name TEXT NOT NULL,
    FOREIGN KEY (employee_id) REFERENCES employees(id)
);

-- Seed data
INSERT INTO employees (id, department, employee_name, salary) VALUES
(1, 'Engineering', 'Ada', 120000),
(2, 'Engineering', 'Linus', 150000),
(3, 'Engineering', 'Grace', 150000),
(4, 'HR', 'Mara', 90000),
(5, 'HR', 'Juno', 92000),
(6, 'HR', 'Theo', 90000);

INSERT INTO projects (id, employee_id, project_name) VALUES
(1, 1, 'Compiler'),
(2, 1, 'Optimizer'),
(3, 2, 'Kernel'),
(4, 4, 'Onboarding Portal'),
(5, 5, 'Recruitment Dashboard'),
(6, 5, 'Employee Stats Pipeline');
