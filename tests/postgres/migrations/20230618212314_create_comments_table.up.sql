-- Add up migration script here
CREATE TABLE comments (
  id SERIAL PRIMARY KEY,
  content TEXT NOT NULL,
  status BOOLEAN NOT NULL
);

INSERT INTO comments (content, status) 
VALUES (
  'This is content test.',
  true
);

INSERT INTO comments (content, status)
VALUES (
  'This is content test 2.',
  false
);
