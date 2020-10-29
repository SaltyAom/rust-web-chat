-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
    name varchar(30) PRIMARY KEY,
    pass varchar(128) NOT NULL
)