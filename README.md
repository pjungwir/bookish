Rust-based personal library database.
Based on the [Diesel tutorial](http://diesel.rs/guides/getting-started/) for Rust.

- Create an empty Postgres database.
- Create `.env` from `.env.sample`.
- `rustup override add nightly`
- `cargo install diesel_cli` if you haven't already.
- `diesel migration run`
- Run these commands:
  - `cargo run --bin add_author`
  - `cargo run --bin publish_author`
  - `cargo run --bin show_authors`
  - `cargo run --bin delete_authors`

Instructions on making it work on stable are at the bottom of the tutorial.

To make a new migration first run:

    diesel migration generate add_blur_to_splurble
    vi migrations/*!$*/*
    diesel migration run

There are also

    diesel migration revert
    diesel migration redo

# TODO

- Write a more general-purpose updatable method.
- Add a books table with a foreign key and more CRUD.
- Use temporal tables.
- Add a shelves table.
- Add a rooms table.
- Write a web interface.
- Search: on author name, book title


# Authors

Paul A. Jungwirth

