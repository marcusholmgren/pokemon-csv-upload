# Pokemon database schema

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
