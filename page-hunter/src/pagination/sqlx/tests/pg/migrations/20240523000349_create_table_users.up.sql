-- Add up migration script here
CREATE TABLE IF NOT EXISTS test_page_hunter.users (
  id SERIAL PRIMARY KEY NOT NULL,
  username VARCHAR(255) NOT NULL,
  hashed_password VARCHAR(255) NOT NULL,
  is_active BOOLEAN NOT NULL DEFAULT TRUE,
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_users_username ON test_page_hunter.users(username);