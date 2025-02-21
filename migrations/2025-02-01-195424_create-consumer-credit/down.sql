-- This file should undo anything in `up.sql`
DROP TABLE IF EXISTS consumer_credit;
DROP TABLE IF EXISTS users;
DROP INDEX IF EXISTS idx_users_username;
DROP INDEX IF EXISTS idx_consumer_credit_consumer_credit_id;
DROP INDEX IF EXISTS idx_consumer_credit_user_id;
