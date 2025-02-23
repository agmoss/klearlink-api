DELETE FROM users WHERE username = 'test_admin';
INSERT INTO users (username, api_key, role, created_at, updated_at)
VALUES ('test_admin', 'test-api-key', 'admin', NOW(), NOW());
