-- Add up migration script here
CREATE TABLE IF NOT EXISTS cars (
    id INT(11) PRIMARY KEY AUTO_INCREMENT,
    model VARCHAR(255) NOT NULL,
    price INT NOT NULL,
    status BOOLEAN NOT NULL DEFAULT TRUE
);

INSERT INTO cars (model, price) VALUES ('BMW', 10000);

CREATE TABLE IF NOT EXISTS orders (
    id INT(11) PRIMARY KEY AUTO_INCREMENT,
    car_id INT NOT NULL,
    buyer VARCHAR(255) NOT NULL,
    price INT NOT NULL,
    status VARCHAR(255) NOT NULL DEFAULT 'pending',
    uuid CHAR(36) NOT NULL,
    order_date DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (car_id) REFERENCES cars (id) ON DELETE CASCADE
);

INSERT INTO orders (car_id, buyer, price, uuid) VALUES (1, 'John Doe', 10000, '77ad72bc-1c3b-4966-98be-828da9888c71');
