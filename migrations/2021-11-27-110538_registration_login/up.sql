-- Create `sys_email_registration` table
CREATE TABLE sys_email_registration (
    code CHAR(10) NOT NULL PRIMARY KEY,
    email VARCHAR(50) NOT NULL UNIQUE CHECK (email LIKE '%@%'), -- Emails must have an @ symbol
    created_on TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);