-- Your SQL goes here
CREATE TABLE consumer_credit_events (
    id SERIAL PRIMARY KEY,
    consumer_credit_id VARCHAR(100) NOT NULL,
    event_type VARCHAR(50) NOT NULL,
    event_data JSONB NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)
