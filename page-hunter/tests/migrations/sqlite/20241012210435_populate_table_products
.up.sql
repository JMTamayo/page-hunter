-- Add up migration script here
INSERT INTO
	products (name, description, price)
VALUES
	('Product 1', 'Description for product 1', 10.0),
	('Product 2', 'Description for product 2', 20.0),
    ('Product 3', 'Description for product 3', 30.0),
    ('Product 4', 'Description for product 4', 40.0),
    ('Product 5', 'Description for product 5', 50.0),
    ('Product 6', 'Description for product 6', 60.0),
    ('Product 7', 'Description for product 7', 70.0),
    ('Product 8', 'Description for product 8', 80.0),
    ('Product 9', 'Description for product 9', 90.0),
    ('Product 10', 'Description for product 10', 100.0)
ON CONFLICT DO NOTHING;