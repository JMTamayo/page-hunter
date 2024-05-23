-- Add up migration script here
CREATE TABLE IF NOT EXISTS test_page_hunter.users (
  id UUID default uuid_generate_v1() PRIMARY KEY NOT NULL,
  username VARCHAR(255) NOT NULL,
  hashed_password VARCHAR(255) NOT NULL,
  is_active BOOLEAN NOT NULL DEFAULT TRUE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_users_username ON test_page_hunter.users(username);

CREATE TABLE IF NOT EXISTS test_page_hunter.addresses (
  id UUID default uuid_generate_v1() PRIMARY KEY NOT NULL,
  user_id UUID NOT NULL,
  address VARCHAR(255) NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ,
  FOREIGN KEY (user_id) REFERENCES test_page_hunter.users(id)
);