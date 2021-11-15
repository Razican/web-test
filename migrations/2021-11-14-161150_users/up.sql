-- Set timezone to UTC
SET TIME ZONE 'UTC';

-- Enable Uuid
CREATE EXTENSION "uuid-ossp";

-- Create the sys_user table
CREATE TABLE sys_user (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    active BOOLEAN NOT NULL,
    username VARCHAR(40) NOT NULL UNIQUE,
    email VARCHAR(50) NOT NULL UNIQUE CHECK (email LIKE '%@%'), -- Emails must have an @ symbol
    password BYTEA NOT NULL,
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    created_on TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_on TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Insert test users
INSERT INTO sys_user
    (active, username, email, password, first_name, last_name)
VALUES
    (TRUE, 'bob', 'bob@example.com', '\x00', 'Bob', 'Smith'),
    (TRUE, 'alice', 'alice@example.com', '\x00', 'Alice', 'Wonderland');