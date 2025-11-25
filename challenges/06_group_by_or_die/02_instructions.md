## Challenge: Group By or Die Tryin

Create a SQLite VIEW named `solution` using grouping sets-like logic (emulated in SQLite) to summarize revenue data at multiple levels of aggregation.

You are given two tables:

* `products(id INTEGER PRIMARY KEY, category TEXT, subcategory TEXT)`
* `sales(product_id INTEGER, amount INTEGER)`

### Your task:

Construct a `VIEW` named `solution` that returns aggregated revenue at the following grouping levels:

1. Category + Subcategory
2. Category only (subcategory should be NULL)
3. Grand total (both category and subcategory should be NULL)

The view must return the following columns:

* `category`
* `subcategory`
* `total_revenue`

Your query must produce exactly these rows and values based on the sample data from the migration.
