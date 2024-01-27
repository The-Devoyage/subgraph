-- Add up migration script here
CREATE TABLE IF NOT EXISTS coffee (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL,
  price INTEGER NOT NULL,
  available BOOLEAN NOT NULL,
  created_by TEXT NOT NULL
);

INSERT INTO coffee (name, price, available, created_by)
VALUES (
  "Katz",
  15,
  true,
  "6510865e93142f6d61b10dd8" --Might need to get a new object id
);

CREATE TABLE IF NOT EXISTS coffee_order (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  coffee_id INTEGER NOT NULL,
  status TEXT NOT NULL DEFAULT "pending",
  created_by TEXT NOT NULL,
  uuid TEXT NOT NULL,
  order_date TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (coffee_id) REFERENCES coffee(id)
);

INSERT INTO coffee_order (coffee_id, created_by, status, uuid)
VALUES (
  1,
  "6510865e93142f6d61b10dd8", --Might need to get a new object id
  "success",
  "af2e25cf-14bc-4e42-9ff1-93a6d3e222af" --Might need to get a new object id
);
