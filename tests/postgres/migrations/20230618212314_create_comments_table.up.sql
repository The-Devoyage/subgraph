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

CREATE TABLE reactions (
  id SERIAL PRIMARY KEY,
  content TEXT NOT NULL,
  status BOOLEAN NOT NULL,
  comment_id INTEGER NOT NULL,
  uuid UUID NOT NULL,
  reaction_date TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO reactions (content, status, comment_id, uuid)
VALUES (
  'This is content test.',
  true,
  1,
  'af2e25cf-14bc-4e42-9ff1-93a6d3e222af'
);
