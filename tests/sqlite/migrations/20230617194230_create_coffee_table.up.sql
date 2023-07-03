-- Add up migration script here
CREATE TABLE IF NOT EXISTS coffee (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL,
  price INTEGER NOT NULL,
  available BOOLEAN NOT NULL
);

INSERT INTO coffee (name, price, available)
VALUES (
  "Katz",
  15,
  true
);
