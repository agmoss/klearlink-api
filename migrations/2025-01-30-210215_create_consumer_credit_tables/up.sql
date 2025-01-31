CREATE TABLE consumer_facts (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    date_of_birth VARCHAR NOT NULL,
    address VARCHAR NOT NULL,
    phone_number VARCHAR NOT NULL,
    consumer_state VARCHAR NOT NULL,
    institution_names TEXT[] NOT NULL
);

CREATE TABLE credit_facts (
    id SERIAL PRIMARY KEY,
    consumer_id INT REFERENCES consumer_facts(id),
    amount FLOAT8 NOT NULL,
    credit_type VARCHAR NOT NULL,
    application_datetime VARCHAR NOT NULL,
    credit_state VARCHAR NOT NULL
);
