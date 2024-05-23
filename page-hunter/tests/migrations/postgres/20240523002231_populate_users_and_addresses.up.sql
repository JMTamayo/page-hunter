-- Add up migration script here
DO $$
DECLARE
    user_id UUID;
BEGIN
    FOR i IN 1..100 LOOP
        INSERT INTO test_page_hunter.users (username, hashed_password, is_active, created_at)
        VALUES ('user' || i, 'hashed_password' || i, TRUE, NOW())
        RETURNING id INTO user_id;

        INSERT INTO test_page_hunter.addresses (user_id, address, created_at, updated_at)
        VALUES (user_id, 'address' || i, NOW(), NOW());
    END LOOP;
END $$;