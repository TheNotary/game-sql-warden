**Challenge:** Implement a SQL query (SQLite-compatible) that returns the **Top 3 highest-scoring players per team**.

You are given a table `players` with the following schema:

```
players(
  id INTEGER PRIMARY KEY,
  team TEXT NOT NULL,
  name TEXT NOT NULL,
  score INTEGER NOT NULL
)
```

Your task:

- Return **exactly the top 3 players per team**, ordered from highest to lowest score *within each team*.
- If a team has fewer than 3 players, return all of them.
- Output columns: `team`, `name`, `score`, and the computed `rank`.

**Important:** SQLite does not support `ROW_NUMBER()` unless using the window extension—assume it *is* available.

Your final query should be written against the schema created in `03_migration.sql`.
