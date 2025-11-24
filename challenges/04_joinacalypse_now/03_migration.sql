-- Schema for Joinocalypse Now

DROP TABLE IF EXISTS employees;
DROP TABLE IF EXISTS departments;
DROP TABLE IF EXISTS projects;
DROP TABLE IF EXISTS assignments;

CREATE TABLE departments (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE employees (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    department_id INTEGER NOT NULL,
    FOREIGN KEY (department_id) REFERENCES departments(id)
);

CREATE TABLE projects (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE assignments (
    employee_id INTEGER NOT NULL,
    project_id INTEGER NOT NULL,
    FOREIGN KEY (employee_id) REFERENCES employees(id),
    FOREIGN KEY (project_id) REFERENCES projects(id)
);

-- Seed data
INSERT INTO departments VALUES
  (1, 'Arcane Research'),
  (2, 'Temporal Logistics'),
  (3, 'Broom Closet Management');

INSERT INTO employees VALUES
  (1, 'Eldon the Thread-Sleeper', 1),
  (2, 'Mira Log-Cleaner', 3),
  (3, 'Tharos of Recursion', 2);

INSERT INTO projects VALUES
  (1, 'The Great Indexing'),
  (2, 'Chrono-Refactor'),
  (3, 'The Mop Initiative');

INSERT INTO assignments VALUES
  (1, 1),
  (1, 2),
  (3, 2),
  (2, 3);
