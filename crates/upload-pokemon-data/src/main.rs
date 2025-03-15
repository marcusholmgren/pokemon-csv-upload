mod db;
mod pokemon_csv;
use db::*;
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
    for result in rdr.deserialize() {
        let record: PokemonCsv = result?;
        let pokemon_row: PokemonTableRow = record.try_into().unwrap();
        println!(
            "{} {:?} {}",
            pokemon_row.pokedex_id, pokemon_row.id, pokemon_row.name
        );
        insert_pokemon(&pool, &pokemon_row).await.unwrap();
    }
    Ok(())
}
