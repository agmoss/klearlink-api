-- Your SQL goes here

DELETE FROM users WHERE username = 'test_admin_user';
INSERT INTO users (username, api_key, role, created_at, updated_at)
VALUES (
    'test_admin_user',
    'c491a813-234a-4bea-b6c4-7413b244dea4',
    'admin',
    NOW(),
    NOW()
);
