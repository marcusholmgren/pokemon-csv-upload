use ksuid::Ksuid;
use sqlx::PgPool;
use std::fmt;

#[derive(Debug)]
pub struct PokemonTableRow {
    pub id: PokemonId,
    pub name: String,
    pub slug: String,
    pub pokedex_id: i16,
    // abilities: Vec<String>,
    // typing: Vec<String>,
    pub hp: i16,
    pub attack: i16,
    pub defense: i16,
    pub special_attack: i16,
    pub special_defense: i16,
    pub speed: i16,
    pub height: i16,
    pub weight: i16,
    pub generation: i16,
    pub female_rate: Option<f32>,
    pub genderless: bool,
    pub legendary_or_mythical: bool,
    pub is_default: bool,
    pub forms_switchable: bool,
    pub base_experience: i16,
    pub capture_rate: i16,
    // egg_groups: Vec<String>,
    pub base_happiness: i16,
    // evolves_from: Option<String>,
    pub primary_color: String,
    pub number_pokemon_with_typing: f32,
    pub normal_attack_effectiveness: f32,
    pub fire_attack_effectiveness: f32,
    pub water_attack_effectiveness: f32,
    pub electric_attack_effectiveness: f32,
    pub grass_attack_effectiveness: f32,
    pub ice_attack_effectiveness: f32,
    pub fighting_attack_effectiveness: f32,
    pub poison_attack_effectiveness: f32,
    pub ground_attack_effectiveness: f32,
    pub fly_attack_effectiveness: f32,
    pub psychic_attack_effectiveness: f32,
    pub bug_attack_effectiveness: f32,
    pub rock_attack_effectiveness: f32,
    pub ghost_attack_effectiveness: f32,
    pub dragon_attack_effectiveness: f32,
    pub dark_attack_effectiveness: f32,
    pub steel_attack_effectiveness: f32,
    pub fairy_attack_effectiveness: f32,
}

pub struct PokemonId(Ksuid);

impl PokemonId {
    /// Creates a new [`PokemonId`].
    pub fn new() -> Self {
        PokemonId(Ksuid::generate())
    }

    pub fn into_bytes(&self) -> Vec<u8> {
        self.0.to_base62().into_bytes()
    }
}

impl fmt::Debug for PokemonId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("PokemonId")
            .field(&self.0.to_base62())
            .finish()
    }
}

pub async fn insert_pokemon(
    pool: &PgPool,
    PokemonTableRow {
        id,
        slug,
        name,
        pokedex_id,
        hp,
        attack,
        defense,
        special_attack,
        special_defense,
        speed,
        height,
        weight,
        generation,
        female_rate,
        genderless,
        legendary_or_mythical,
        is_default,
        forms_switchable,
        base_experience,
        capture_rate,
        base_happiness,
        primary_color,
        number_pokemon_with_typing,
        normal_attack_effectiveness,
        fire_attack_effectiveness,
        water_attack_effectiveness,
        electric_attack_effectiveness,
        grass_attack_effectiveness,
        ice_attack_effectiveness,
        fighting_attack_effectiveness,
        poison_attack_effectiveness,
        ground_attack_effectiveness,
        fly_attack_effectiveness,
        psychic_attack_effectiveness,
        bug_attack_effectiveness,
        rock_attack_effectiveness,
        ghost_attack_effectiveness,
        dragon_attack_effectiveness,
        dark_attack_effectiveness,
        steel_attack_effectiveness,
        fairy_attack_effectiveness,
    }: &PokemonTableRow,
) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
        insert into pokemon (id, slug, name, pokedex_id, hp, attack, defense, special_attack, special_defense, speed,
                                    height, weight, generation, female_rate, genderless, legendary_or_mythical, is_default,
                                    forms_switchable, base_experience, capture_rate, base_happiness, primary_color,
                                    number_pokemon_with_typing, normal_attack_effectiveness, fire_attack_effectiveness,
                                    water_attack_effectiveness, electric_attack_effectiveness, grass_attack_effectiveness,
                                    ice_attack_effectiveness, fighting_attack_effectiveness, poison_attack_effectiveness,
                                    ground_attack_effectiveness, fly_attack_effectiveness, psychic_attack_effectiveness,
                                    bug_attack_effectiveness, rock_attack_effectiveness, ghost_attack_effectiveness,
                                    dragon_attack_effectiveness, dark_attack_effectiveness, steel_attack_effectiveness,
                                    fairy_attack_effectiveness)
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31, $32, $33, $34, $35, $36, $37, $38, $39, $40, $41);
        "#,
        id.into_bytes(),
        slug,
        name,
        pokedex_id,
        hp,
        attack,
        defense,
        special_attack,
        special_defense,
        speed,
        height,
        weight,
        generation,
        female_rate.unwrap_or(0.0),
        genderless,
        legendary_or_mythical,
        is_default,
        forms_switchable,
        base_experience,
        capture_rate,
        base_happiness,
        primary_color,
        number_pokemon_with_typing,
        normal_attack_effectiveness,
        fire_attack_effectiveness,
        water_attack_effectiveness,
        electric_attack_effectiveness,
        grass_attack_effectiveness,
        ice_attack_effectiveness,
        fighting_attack_effectiveness,
        poison_attack_effectiveness,
        ground_attack_effectiveness,
        fly_attack_effectiveness,
        psychic_attack_effectiveness,
        bug_attack_effectiveness,
        rock_attack_effectiveness,
        ghost_attack_effectiveness,
        dragon_attack_effectiveness,
        dark_attack_effectiveness,
        steel_attack_effectiveness,
        fairy_attack_effectiveness,
    )
    .execute(pool)
    .await
}
