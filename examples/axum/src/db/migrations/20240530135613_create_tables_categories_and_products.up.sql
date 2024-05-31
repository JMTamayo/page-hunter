-- Add up migration script here
CREATE TABLE IF NOT EXISTS inventory.categories (
	id UUID default uuid_generate_v1() PRIMARY KEY NOT NULL,
	name VARCHAR(255) NOT NULL UNIQUE,
	created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

COMMENT ON TABLE inventory.categories IS 'This table stores product categories.';
COMMENT ON COLUMN inventory.categories.id IS 'Unique identifier for the category.';
COMMENT ON COLUMN inventory.categories.name IS 'Name of the category.';
COMMENT ON COLUMN inventory.categories.created_at IS 'Timestamp when the category was created.';

CREATE TABLE IF NOT EXISTS inventory.products (
	id UUID default uuid_generate_v1() PRIMARY KEY NOT NULL,
	name VARCHAR(255) NOT NULL UNIQUE,
	description VARCHAR NOT NULL,
	quantity INT NOT NULL,
	unitary_price FLOAT NOT NULL,
	category_id UUID NOT NULL,
	created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_at TIMESTAMPTZ,
	foreign key (category_id) references inventory.categories(id)
);

COMMENT ON TABLE inventory.products IS 'This table stores products and their related information.';
COMMENT ON COLUMN inventory.products.id IS 'Unique identifier for the product.';
COMMENT ON COLUMN inventory.products.name IS 'Name of the product.';
COMMENT ON COLUMN inventory.products.description IS 'Description of the product.';
COMMENT ON COLUMN inventory.products.quantity IS 'Quantity of the product in stock.';
COMMENT ON COLUMN inventory.products.unitary_price IS 'Unit price of the product.';
COMMENT ON COLUMN inventory.products.category_id IS 'Category ID the product belongs to.';
COMMENT ON COLUMN inventory.products.created_at IS 'Timestamp when the product was created.';
COMMENT ON COLUMN inventory.products.updated_at IS 'Timestamp when the product was last updated.';