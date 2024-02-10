-- Your SQL goes here
CREATE TABLE accounts (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  description TEXT,
  account_type VARCHAR,
  protocol VARCHAR,
  login VARCHAR,
  password VARCHAR,
  host VARCHAR,
  port VARCHAR,
  url VARCHAR
)