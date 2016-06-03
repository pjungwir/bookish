CREATE TABLE authors (
  id SERIAL PRIMARY KEY,
  first_name TEXT NOT NULL,
  last_name TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 'f',
  created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
  updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL
);
CREATE INDEX idx_authors_first_last ON authors (first_name, last_name);
CREATE INDEX idx_authors_last_first ON authors (last_name, first_name);

