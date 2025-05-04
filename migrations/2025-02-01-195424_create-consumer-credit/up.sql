-- Your SQL goes here

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(100) NOT NULL,
    api_key UUID NOT NULL,
    role VARCHAR(100) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP,
    UNIQUE (username),
    UNIQUE (api_key)
);

CREATE TABLE consumer_credit (
    id SERIAL PRIMARY KEY,
    consumer_credit_id VARCHAR(100) NOT NULL,
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    email VARCHAR(320) NOT NULL CHECK (
        email ~ '^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$'
    ),
    date_of_birth DATE NOT NULL,
    address TEXT NOT NULL,
    phone_number VARCHAR(20) NOT NULL CHECK (phone_number ~ '^\+?[0-9\s-]+$'),
    sin_ssn VARCHAR(11) UNIQUE CHECK (
        sin_ssn ~ '^\d{3}-\d{3}-\d{3}$'
        OR sin_ssn ~ '^\d{3}-\d{2}-\d{4}$'
    ),
    institution_names TEXT [] NOT NULL,
    amount FLOAT NOT NULL CHECK (amount >= 0),
    credit_type VARCHAR(10) NOT NULL CHECK (credit_type IN ('PDL', 'BNPL')),
    application_datetime TIMESTAMP NOT NULL,
    originated_datetime TIMESTAMP,
    payment_due_date TIMESTAMP,
    payment_due_amount FLOAT CHECK (amount >= 0),
    credit_state VARCHAR(20) NOT NULL CHECK (
        credit_state IN (
            'application',
            'originated',
            'declined',
            'non-compliant',
            'compliant'
        )
    ),
    consumer_information_indicator VARCHAR(2),
    user_id INTEGER NOT NULL REFERENCES users (id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP,
    -- BNPL specific fields
    total_installments INTEGER CHECK (total_installments > 0),
    paid_installments INTEGER CHECK (paid_installments >= 0 AND paid_installments <= total_installments),
    installment_amount FLOAT CHECK (installment_amount >= 0),
    UNIQUE (consumer_credit_id, user_id)
);

CREATE INDEX idx_users_username ON users (username);
CREATE INDEX idx_consumer_credit_consumer_credit_id ON consumer_credit (
    consumer_credit_id
);
CREATE INDEX idx_consumer_credit_user_id ON consumer_credit (user_id);

SELECT DIESEL_MANAGE_UPDATED_AT('users');
SELECT DIESEL_MANAGE_UPDATED_AT('consumer_credit');
