-- Your SQL goes here

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL UNIQUE,
  birth_date DATE NOT NULL
)