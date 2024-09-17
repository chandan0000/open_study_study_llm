-- Step 1: Create an ENUM type for the 'role' field
CREATE TYPE user_role AS ENUM ('admin', 'user');

-- Step 2: Create the 'users' table
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    role user_role NOT NULL,
    fullname VARCHAR(255) NOT NULL,
    social_media_link VARCHAR(255),
    delete_account_date TIMESTAMP,
    update_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    create_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Step 3: Create a function to update 'update_date' on record modification
CREATE OR REPLACE FUNCTION update_modified_column()
RETURNS TRIGGER AS $$
BEGIN
   NEW.update_date = NOW();
   RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Step 4: Create a trigger that calls the function before each update
CREATE TRIGGER update_users_modtime
BEFORE UPDATE ON users
FOR EACH ROW
EXECUTE PROCEDURE update_modified_column();