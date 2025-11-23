## Challenge: Conditional Logic in SQLite

You are given a table named **heroes** representing individuals in the dungeon, along with their current health.

Your task:  
Create a **VIEW named `solution`** that returns each hero’s `name` and a computed column called **`status`**.

Rules for determining `status`:

- If a hero’s health is **greater than 70**, `status` must be `"strong"`.
- If a hero's health is **between 31 and 70 inclusive**, `status` must be `"wounded"`.
- If a hero's health is **30 or below**, `status` must be `"critical"`.

You must use **SQLite conditional logic**, such as `CASE`.

### Expected Columns in `solution`
| Column | Description |
|--------|-------------|
| name   | The hero's name |
| status | Text indicating strong / wounded / critical |

Do **not** modify the base table.  
Do **not** include extra columns.  
Only create the VIEW `solution` that applies this logic.

