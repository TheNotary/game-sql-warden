-- Expect these rows:
-- Team Red: Auron(95, rank 1), Brant(92, rank 2), Selka(88, rank 3)
-- Team Blue: Nira(99, rank 1), Kade(85, rank 2), Milo(85, rank 3)
-- Team Green: Pox(10, rank 1)

WITH expected AS (
  SELECT 'Red' AS team, 'Auron' AS name, 95 AS score, 1 AS rank UNION ALL
  SELECT 'Red', 'Brant', 92, 2 UNION ALL
  SELECT 'Red', 'Selka', 88, 3 UNION ALL
  SELECT 'Blue', 'Nira', 99, 1 UNION ALL
  SELECT 'Blue', 'Kade', 85, 2 UNION ALL
  SELECT 'Blue', 'Milo', 85, 3 UNION ALL
  SELECT 'Green', 'Pox', 10, 1
)
SELECT
  CASE WHEN EXISTS (
    SELECT *
    FROM solution s
    FULL OUTER JOIN expected e
      ON s.team = e.team AND s.name = e.name AND s.score = e.score AND s.rank = e.rank
    WHERE s.team IS NULL OR e.team IS NULL
  ) THEN 'FAIL' ELSE 'PASS' END AS result;

