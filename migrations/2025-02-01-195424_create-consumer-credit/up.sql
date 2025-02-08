-- Your SQL goes here
CREATE TABLE consumer_credit (
    id SERIAL PRIMARY KEY,
    consumer_credit_id VARCHAR(100) NOT NULL,
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    email VARCHAR(320) NOT NULL CHECK (email ~ '^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$'), -- RFC 5322 & 822-compliant regex check
    date_of_birth DATE NOT NULL, -- Ensuring proper date handling with DATE type
    address TEXT NOT NULL,
    phone_number VARCHAR(20) NOT NULL CHECK (phone_number ~ '^\+?[0-9\s-]+$'), -- Supports E.164 format
    sin_ssn VARCHAR(11) UNIQUE CHECK (sin_ssn ~ '^\d{3}-\d{3}-\d{3}$' OR sin_ssn ~ '^\d{3}-\d{2}-\d{4}$'), -- Optional SIN/SSN with pattern validation
    institution_names TEXT[] NOT NULL,
    amount NUMERIC(12, 2) NOT NULL CHECK (amount >= 0), -- Ensures accurate financial calculations
    credit_type VARCHAR(10) NOT NULL CHECK (credit_type IN ('PDL', 'BNPL')), -- Enforce valid credit types
    application_datetime TIMESTAMP NOT NULL, -- Stores ISO 8601 datetime accurately
    credit_state VARCHAR(20) NOT NULL CHECK (credit_state IN (
        'application', 'originated', 'declined', 'non-compliant', 'compliant', 'bankrupt/insolvent'
    )),
    tenant VARCHAR(50) NOT NULL,
    UNIQUE (consumer_credit_id, tenant)
);
