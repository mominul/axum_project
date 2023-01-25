-- Add migration script here

CREATE TABLE accounts (
  user_name VARCHAR PRIMARY KEY UNIQUE NOT NULL,
  password VARCHAR NOT NULL
);