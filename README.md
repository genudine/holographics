# Genudine Holographics

**Actively WIP**. Shit will break. Help is appreciated!

Daybreak Games Census as a GraphQL service.

Thanks to https://github.com/PS2Sanctuary/Sanctuary.Census folks for the augmented API data!

This is not currently deployed to the internet.

## Developing

You need:

- Rust 1.60.0+ (get it via https://rustup.rs)
- Redis
  - You can run `docker compose up -d` to get one running.

You run:

- `cargo run`

yeet 🦀

### Structure

Everything that's real data lives in `src/collections`. Each collection definition will need an Object, an impl method that returns Self, and a Query definition. A good base is to start from `src/collections/title.rs`, which pulls title simple data from Sanctuary.

Each new collection must then:

- Add it's Query object to `src/query.rs`
- Remember to add the `pub mod _my_collection` to `src/collections/mod.rs`
