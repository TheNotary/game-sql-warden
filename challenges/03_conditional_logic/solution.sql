DROP VIEW IF EXISTS solution;

CREATE VIEW solution AS
SELECT name,
  CASE
       WHEN health > 70 THEN "strong"
       WHEN health > 30 THEN "wounded"
       ELSE "critical"
  END AS status
  FROM heroes;
