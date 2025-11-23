## 🧩 The Challenge

After running the migrations below, write a query that produces a table with:

    cube_id | cube_name | monster_id | monster_name | hp

…containing the strongest (highest HP) monster in each cube.
If a cube has no monsters, it should not appear (inner semantics).

Create your solution as a view named strongest_monsters.

## Technical Instructions

    sqlite3 test.db < migration.sql
    sqlite3 test.db
