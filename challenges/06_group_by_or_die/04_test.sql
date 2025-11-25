WITH expected AS (
  SELECT 'Tech' AS category, 'Phones' AS subcategory, 500 AS total_revenue
  UNION ALL SELECT 'Tech', 'Laptops', 1000
  UNION ALL SELECT 'Home', 'Kitchen', 200
  UNION ALL SELECT 'Home', 'Garden', 500
  UNION ALL SELECT 'Tech', NULL, 1500
  UNION ALL SELECT 'Home', NULL, 700
  UNION ALL SELECT NULL, NULL, 2200
),
check_rows AS (
  SELECT
    CASE WHEN (
      SELECT COUNT(*) FROM (
        SELECT * FROM solution
        EXCEPT
        SELECT * FROM expected
      )
      +
      (
        SELECT COUNT(*) FROM (
          SELECT * FROM expected
          EXCEPT
          SELECT * FROM solution
        )
      ) = 0
    THEN 'PASS' ELSE 'FAIL' END AS result
)
SELECT result FROM check_rows;
