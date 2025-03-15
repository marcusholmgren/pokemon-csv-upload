use crate::pokemon_csv::PokemonCsv;
use std::fmt;

use inflector::Inflector;
use ksuid::Ksuid;

#[derive(Debug)]
pub struct PokemonTableRow {
    pub id: PokemonId,
    pub name: String,
    pub slug: String,
    pub pokedex_id: u16,
    // abilities: Vec<String>,
    // typing: Vec<String>,
    pub hp: u16,
    pub attack: u16,
    pub defense: u16,
    pub special_attack: u16,
    pub special_defense: u16,
    pub speed: u16,
    pub height: u16,
    pub weight: u16,
    pub generation: u16,
    pub female_rate: Option<f32>,
    pub genderless: bool,
    pub legendary_or_mythical: bool,
    pub is_default: bool,
    pub forms_switchable: bool,
    pub base_experience: u16,
    pub capture_rate: u16,
    // egg_groups: Vec<String>,
    pub base_happiness: u16,
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
    pub fn new() -> Self {
        PokemonId(Ksuid::generate())
    }
}

impl fmt::Debug for PokemonId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("PokemonId")
            .field(&self.0.to_base62())
            .finish()
    }
}

impl From<PokemonCsv> for PokemonTableRow {
    fn from(csv: PokemonCsv) -> Self {
        PokemonTableRow {
            id: PokemonId::new(),
            slug: csv.name.to_kebab_case(),
            name: csv.name,
            pokedex_id: csv.pokedex_id,
            hp: csv.hp.into(),
            attack: csv.attack.into(),
            defense: csv.defense.into(),
            special_attack: csv.special_attack.into(),
            special_defense: csv.special_defense.into(),
            speed: csv.speed.into(),
            height: csv.height,
            weight: csv.weight,
            generation: csv.generation.into(),
            female_rate: csv.female_rate,
            genderless: csv.genderless,
            legendary_or_mythical: csv.is_legendary_or_mythical,
            is_default: csv.is_default,
            forms_switchable: csv.forms_switchable,
            base_experience: csv.base_experience.into(),
            capture_rate: csv.capture_rate.into(),
            base_happiness: csv.base_happiness.into(),
            primary_color: csv.primary_color,
            number_pokemon_with_typing: csv.number_pokemon_with_typing,
            normal_attack_effectiveness: csv.normal_attack_effectiveness,
            fire_attack_effectiveness: csv.fire_attack_effectiveness,
            water_attack_effectiveness: csv.water_attack_effectiveness,
            electric_attack_effectiveness: csv.electric_attack_effectiveness,
            grass_attack_effectiveness: csv.grass_attack_effectiveness,
            ice_attack_effectiveness: csv.ice_attack_effectiveness,
            fighting_attack_effectiveness: csv.fighting_attack_effectiveness,
            poison_attack_effectiveness: csv.poison_attack_effectiveness,
            ground_attack_effectiveness: csv.ground_attack_effectiveness,
            fly_attack_effectiveness: csv.fly_attack_effectiveness,
            psychic_attack_effectiveness: csv.psychic_attack_effectiveness,
            bug_attack_effectiveness: csv.bug_attack_effectiveness,
            rock_attack_effectiveness: csv.rock_attack_effectiveness,
            ghost_attack_effectiveness: csv.ghost_attack_effectiveness,
            dragon_attack_effectiveness: csv.dragon_attack_effectiveness,
            dark_attack_effectiveness: csv.dark_attack_effectiveness,
            steel_attack_effectiveness: csv.steel_attack_effectiveness,
            fairy_attack_effectiveness: csv.fairy_attack_effectiveness,
        }
    }
}
