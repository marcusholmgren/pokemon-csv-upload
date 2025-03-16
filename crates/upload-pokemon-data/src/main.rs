mod db;
mod pokemon_csv;
use db::*;
use indicatif::ProgressIterator;
use pokemon_csv::*;
use sqlx::postgres::PgPoolOptions;
use std::{collections::HashMap, env};

#[tokio::main]
async fn main() -> Result<(), csv::Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    let mut pokemon_map: HashMap<String, PokemonId> = HashMap::new();

    let mut rdr = csv::Reader::from_path("./crates/upload-pokemon-data/pokemon.csv")?;
    let pokemon = rdr
        .deserialize()
        .collect::<Result<Vec<PokemonCsv>, csv::Error>>()?;
    for record in pokemon.clone().into_iter().progress() {
        let pokemon_row: PokemonTableRow = record.clone().into();
        insert_pokemon(&pool, &pokemon_row).await.unwrap();
        for ability in record.abilities.iter() {
            insert_abilities(&pool, &pokemon_row.id, ability)
                .await
                .unwrap();
        }
        for egg_group in record.egg_groups.iter() {
            insert_egg_groups(&pool, &pokemon_row.id, egg_group)
                .await
                .unwrap();
        }
        for typing in record.typing.iter() {
            insert_typing(&pool, &pokemon_row.id, typing).await.unwrap();
        }
        pokemon_map.insert(record.name.clone(), pokemon_row.id);
    }

    for pokemon in pokemon
        .into_iter()
        .progress()
        .filter(|pokemon| pokemon.evolves_from.is_some())
    {
        let name = pokemon
            .evolves_from
            .expect("Expected a value here since we just checked");
        let pokemon_id = pokemon_map.get(&pokemon.name);
        let evolves_from = pokemon_map.get(&name);
        if let (Some(pokemon_id), Some(evolves_from)) = (pokemon_id, evolves_from) {
            insert_evolutions(&pool, pokemon_id, evolves_from)
                .await
                .unwrap();
        }
    }
    Ok(())
}
