/* Create table for a PostgreSQL database */

CREATE TABLE IF NOT EXISTS pokemon (
    id BYTEA NOT NULL, -- ksuid
    slug VARCHAR(30) NOT NULL, -- generated
    name VARCHAR(30) NOT NULL,
    pokedex_id SMALLINT NOT NULL,
    -- abilities -- new table
    -- typing -- new table
    hp SMALLINT NOT NULL,
    attack SMALLINT NOT NULL,
    defense SMALLINT NOT NULL,
    special_attack SMALLINT NOT NULL,
    special_defense SMALLINT NOT NULL,
    speed SMALLINT NOT NULL,
    height SMALLINT NOT NULL,
    weight SMALLINT NOT NULL,
    generation SMALLINT NOT NULL,
    female_rate REAL,
    genderless BOOLEAN NOT NULL,
    legendary_or_mythical BOOLEAN NOT NULL,
    is_default BOOLEAN NOT NULL,
    forms_switchable BOOLEAN NOT NULL,
    base_experience SMALLINT NOT NULL,
    capture_rate SMALLINT NOT NULL,
    -- egg_groups -- new table
    base_happiness SMALLINT NOT NULL,
    -- evolves_from -- new table
    primary_color VARCHAR(6) NOT NULL,
    number_pokemon_with_typing REAL NOT NULL,
    normal_attack_effectiveness REAL NOT NULL,
    fire_attack_effectiveness REAL NOT NULL,
    water_attack_effectiveness REAL NOT NULL,
    electric_attack_effectiveness REAL NOT NULL,
    grass_attack_effectiveness REAL NOT NULL,
    ice_attack_effectiveness REAL NOT NULL,
    fighting_attack_effectiveness REAL NOT NULL,
    poison_attack_effectiveness REAL NOT NULL,
    ground_attack_effectiveness REAL NOT NULL,
    fly_attack_effectiveness REAL NOT NULL,
    psychic_attack_effectiveness REAL NOT NULL,
    bug_attack_effectiveness REAL NOT NULL,
    rock_attack_effectiveness REAL NOT NULL,
    ghost_attack_effectiveness REAL NOT NULL,
    dragon_attack_effectiveness REAL NOT NULL,
    dark_attack_effectiveness REAL NOT NULL,
    steel_attack_effectiveness REAL NOT NULL,
    fairy_attack_effectiveness REAL NOT NULL,
    PRIMARY KEY (id),
    UNIQUE (slug)
);
