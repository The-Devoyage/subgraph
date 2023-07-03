-- Add up migration script here
CREATE TABLE IF NOT EXISTS cars (
    id SERIAL PRIMARY KEY,
    model VARCHAR(255) NOT NULL,
    price INT NOT NULL,
    status BOOLEAN NOT NULL DEFAULT TRUE
);

INSERT INTO cars (model, price) VALUES ('BMW', 10000);
