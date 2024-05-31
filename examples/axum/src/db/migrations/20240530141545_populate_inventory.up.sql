-- Add up migration script here
-- Populate categories table:
INSERT INTO inventory.categories (id, name) 
VALUES 
    ('17801a41-0f97-46d1-809f-2fa3e301f9ea', 'Fresh Produce'), 
    ('8980d63e-7e1d-4b6e-97d8-67e7adbd473e', 'Dairy & Eggs'), 
    ('58740dc5-0b52-45c9-a3b1-a852a1e6f03d', 'Meat & Seafood'),
    ('023366b8-fc36-41ed-978a-a1c5856a08eb', 'Bakery')
ON CONFLICT DO NOTHING;

-- Populate products table:
INSERT INTO inventory.products (name, description, quantity, unitary_price, category_id)
VALUES
	('Apples', 'Fresh apples', 100, 0.5, '17801a41-0f97-46d1-809f-2fa3e301f9ea'),
	('Oranges', 'Fresh oranges', 80, 0.6, '17801a41-0f97-46d1-809f-2fa3e301f9ea'),
	('Milk', '1L milk', 50, 1.2, '8980d63e-7e1d-4b6e-97d8-67e7adbd473e'),
	('Cheese', 'Cheddar cheese', 30, 2.5, '8980d63e-7e1d-4b6e-97d8-67e7adbd473e'),
	('Chicken', 'Fresh chicken', 20, 5.0, '58740dc5-0b52-45c9-a3b1-a852a1e6f03d'),
	('Fish', 'Fresh fish', 15, 7.0, '58740dc5-0b52-45c9-a3b1-a852a1e6f03d'),
	('Bread', 'Fresh bread', 100, 1.0, '023366b8-fc36-41ed-978a-a1c5856a08eb'),
	('Croissant', 'Fresh croissant', 50, 1.5, '023366b8-fc36-41ed-978a-a1c5856a08eb'),
	('Eggs', 'Dozen eggs', 40, 2.0, '8980d63e-7e1d-4b6e-97d8-67e7adbd473e'),
	('Butter', '500g butter', 30, 2.5, '8980d63e-7e1d-4b6e-97d8-67e7adbd473e')
ON CONFLICT DO NOTHING;