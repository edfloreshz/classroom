CREATE TABLE users (
    id TEXT PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    first_name TEXT NOT NULL,
    last_name TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO users
(
    id,
    username,
    email,
    password,
    first_name,
    last_name
)
VALUES
(
    '6efc59f5-39ae-4629-8e51-10ce3ead5cab',
    'johndoe',
    'john@doe.com',
    'john123',
    'John',
    'Doe'
);
