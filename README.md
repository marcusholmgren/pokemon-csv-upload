# Pokémon CVS upload

This is a project to read a CSV file and upload it to a database.

The schema for this project can viewed in the [database README](./db/README.md)

## Features

- Read Pokémon CSV files
- Upload to a Postgres database

## Run Locally

To run this project locally, you will need to have rust installed. You can install it from [here](https://www.rust-lang.org/tools/install)
You also need to have a Postgres database running. You can use the provided docker `compose.yaml` file to start a Postgres database.

```bash
  docker-compose up
```

```bash
  cargo run --bin upload-pokemon-data
```

## Acknowledgements

 - [Rust Adventure](https://www.rustadventure.dev) by Chris Biscardi for the original idea and the tutorial
