mod db;
mod pokemon_csv;
use db::*;
use indicatif::ProgressIterator;
use pokemon_csv::*;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() -> Result<(), csv::Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    let mut rdr = csv::Reader::from_path("./crates/upload-pokemon-data/pokemon.csv")?;
    let pokemon = rdr
        .deserialize()
        .collect::<Result<Vec<PokemonCsv>, csv::Error>>()?;
    for record in pokemon.into_iter().progress() {
        let pokemon_row: PokemonTableRow = record.into();
        insert_pokemon(&pool, &pokemon_row).await.unwrap();
    }
    Ok(())
}
