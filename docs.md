# Pokémon Library Documentation

A Rust library for retrieving Pokémon names and information in multiple languages.

## Overview

The `pokemon` crate provides a simple interface to access Pokémon data from a CSV database. It supports retrieving Pokémon names in 9 different languages, including Japanese (both literal and transliterated), Korean, Chinese, French, German, Spanish, Italian, and English.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
pokemon = "1.0.0"
```

Or install via cargo:

```bash
cargo install pokemon
```

## Data Structure

### Pokemon Struct

Each Pokémon is represented by a `Pokemon` struct with the following fields:

| Field    | Type           | Description                                    |
|----------|----------------|------------------------------------------------|
| species  | `i32`          | The Pokémon species ID (1-1025)               |
| language | `i32`          | The language ID (see Language Codes below)     |
| name     | `String`       | The Pokémon's name in the specified language  |
| genus    | `Option<String>` | The Pokémon's genus/category (e.g., "Seed")    |

### Language Codes

The library supports the following languages, identified by their language IDs:

| ID | Language                  |
|----|---------------------------|
| 1  | Japanese (literal)        |
| 2  | Japanese (transliterated) |
| 3  | Korean                    |
| 4  | Chinese                   |
| 5  | French                    |
| 6  | German                    |
| 7  | Spanish                   |
| 8  | Italian                   |
| 9  | English                   |

## API Reference

### `pokemon::get_all()`

Returns a vector containing all Pokémon entries across all languages.

**Returns:** `Vec<Pokemon>`

**Example:**
```rust
use pokemon::pokemon;

let all_pokemon = pokemon::get_all();
println!("Total entries: {}", all_pokemon.len());
```

### `pokemon::get_pokemon(id, lang)`

Returns a specific Pokémon based on its species ID and language.

**Parameters:**
- `id: usize` - The Pokémon species ID (1-1025)
- `lang_id: i32` - The language ID (1-9)

**Returns:** `Pokemon`

**Example:**
```rust
use pokemon::pokemon;

// Get Bulbasaur in English
let bulbasaur = pokemon::get_pokemon(1, 9);
println!("{}", bulbasaur.name); // Prints "Bulbasaur"

// Get Bulbasaur in French
let bulbasaur_fr = pokemon::get_pokemon(1, 5);
println!("{}", bulbasaur_fr.name); // Prints "Bulbizarre"
```

### `pokemon::get_random()`

Returns a random Pokémon in English.

**Returns:** `Pokemon`

**Example:**
```rust
use pokemon::pokemon;

let random_pokemon = pokemon::get_random();
println!("Random Pokémon: {}", random_pokemon.name);
```

### `pokemon::get_random_with_lang(lang_id)`

Returns a random Pokémon in the specified language.

**Parameters:**
- `lang_id: i32` - The language ID (1-9)

**Returns:** `Pokemon`

**Example:**
```rust
use pokemon::pokemon;

// Get a random Pokémon in Japanese
let random_jp = pokemon::get_random_with_lang(1);
println!("Random Pokémon (Japanese): {}", random_jp.name);
```

### `pokemon::get_name(id)`

Returns the name of a Pokémon by its species ID in English.

**Parameters:**
- `id: usize` - The Pokémon species ID (1-1025)

**Returns:** `String`

**Example:**
```rust
use pokemon::pokemon;

let name = pokemon::get_name(9);
println!("{}", name); // Prints "Blastoise"
```

### `pokemon::get_name_with_lang(id, lang_id)`

Returns the name of a Pokémon by its species ID in the specified language.

**Parameters:**
- `id: usize` - The Pokémon species ID (1-1025)
- `lang_id: i32` - The language ID (1-9)

**Returns:** `String`

**Example:**
```rust
use pokemon::pokemon;

// Get Pikachu's name in different languages
let pikachu_en = pokemon::get_name_with_lang(25, 9); // "Pikachu"
let pikachu_fr = pokemon::get_name_with_lang(25, 5); // "Pikachu"
let pikachu_jp = pokemon::get_name_with_lang(25, 1); // "ピカチュウ"
```

## Usage Examples

### Basic Usage

```rust
extern crate pokemon;

use pokemon::pokemon;

fn main() {
    // Get a specific Pokémon
    let charizard = pokemon::get_pokemon(6, 9);
    println!("{} - {}", charizard.name, charizard.genus.unwrap_or_default());
    
    // Get just the name
    let name = pokemon::get_name(25);
    println!("Pokémon #25: {}", name);
}
```

### Multi-language Support

```rust
extern crate pokemon;

use pokemon::pokemon;

fn main() {
    let pokemon_id = 1; // Bulbasaur
    
    println!("Bulbasaur in different languages:");
    println!("English: {}", pokemon::get_name_with_lang(pokemon_id, 9));
    println!("French: {}", pokemon::get_name_with_lang(pokemon_id, 5));
    println!("German: {}", pokemon::get_name_with_lang(pokemon_id, 6));
    println!("Spanish: {}", pokemon::get_name_with_lang(pokemon_id, 7));
    println!("Japanese: {}", pokemon::get_name_with_lang(pokemon_id, 1));
}
```

### Random Pokémon Generator

```rust
extern crate pokemon;

use pokemon::pokemon;

fn main() {
    // Get 5 random Pokémon
    for _ in 0..5 {
        let random = pokemon::get_random();
        println!("Random Pokémon: {} (ID: {})", random.name, random.species);
    }
}
```

MIT License.
