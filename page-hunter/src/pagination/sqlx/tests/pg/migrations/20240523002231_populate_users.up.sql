-- Add up migration script here
DO $$
DECLARE
    user_id INTEGER;
BEGIN
    FOR i IN 1..100 LOOP
        INSERT INTO test_page_hunter.users (id, username, hashed_password, is_active)
        VALUES (i, 'user' || i, 'hashed_password' || i, TRUE)
        RETURNING id INTO user_id;
    END LOOP;
END $$;