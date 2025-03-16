# Pokémon database schema

The `schema.sql` file contains the schema for the Pokemon database.
There are 5 tables in the database:
- abilities
- egg_groups
- evolutions
- pokemon
- typing

The schema is as follows:

```mermaid
classDiagram
   direction BT
   class abilities {
      bytea pokemon_id
   }
   class egg_groups {
      bytea pokemon_id
   }
   class evolutions {
      bytea pokemon_id
   }
   class pokemon {
      bytea id
   }
   class typing {
      bytea pokemon_id
   }

   abilities  -->  pokemon : pokemon_id
   egg_groups  -->  pokemon : pokemon_id
   evolutions  -->  pokemon : pokemon_id
   typing  -->  pokemon : pokemon_id
```


## Analytical queries

What evelutions are there for each pokémon?

```sql
SELECT
    b.name AS pokemon,
    string_agg(a.name, ', ') AS evolves_into
FROM
    evolutions
LEFT JOIN
    pokemon a ON a.id = evolutions.pokemon_id
LEFT JOIN
    pokemon b ON b.id = evolutions.evolves_from
GROUP BY
    b.name
order by count(b.name) desc;
```
