DROP TABLE IF EXISTS products;
DROP TABLE IF EXISTS sales;
DROP VIEW IF EXISTS solution;

CREATE TABLE products (
  id INTEGER PRIMARY KEY,
  category TEXT,
  subcategory TEXT
);

CREATE TABLE sales (
  product_id INTEGER,
  amount INTEGER
);

INSERT INTO products(id, category, subcategory) VALUES
  (1, 'Tech', 'Phones'),
  (2, 'Tech', 'Laptops'),
  (3, 'Home', 'Kitchen'),
  (4, 'Home', 'Garden');

INSERT INTO sales(product_id, amount) VALUES
  (1, 300), (1, 200),
  (2, 1000),
  (3, 150), (3, 50),
  (4, 500);
