CREATE TABLE books (
  id SERIAL PRIMARY KEY,
  title TEXT NOT NULL,
  published_at TIMESTAMP WITHOUT TIME ZONE,
  created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
  updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL
);

CREATE INDEX idx_books_published_at ON books (published_at);

CREATE TABLE authorships (
  id SERIAL PRIMARY KEY,
  author_id INTEGER REFERENCES authors (id),
  book_id INTEGER REFERENCES books (id),
  created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
  updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL
);

CREATE INDEX idx_authorships_author_book ON authorships (author_id, book_id);
CREATE INDEX idx_authorships_book_author ON authorships (book_id, author_id);

