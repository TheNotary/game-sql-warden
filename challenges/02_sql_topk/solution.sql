DROP VIEW IF EXISTS solution;

CREATE VIEW solution AS
WITH ranked AS (
    SELECT
        *,
        ROW_NUMBER() OVER (PARTITION BY team ORDER BY score DESC) AS rank
    FROM players
)
SELECT *
  FROM ranked
 WHERE rank <= 3
 ORDER BY team, rank ASC;
