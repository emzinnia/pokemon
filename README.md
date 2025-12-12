# Pokémon ![premierball](media/premier.png)

[![CI](https://github.com/emzinnia/pokemon/actions/workflows/ci.yml/badge.svg)](https://github.com/emzinnia/pokemon/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/pokemon.svg?style=flat-square)](https://crates.io/crates/pokemon)
![contributions](https://img.shields.io/badge/contributions-welcome-brightgreen.svg?style=flat-square)

### [Master](https://github.com/emzinnia/pokemon/tree/master) | [Stable](https://github.com/emzinnia/pokemon/tree/stable) | [Dev](https://github.com/emzinnia/pokemon/tree/master)

A tiny Rust library inspired by [Pokémon](https://github.com/sindresorhus/pokemon), that lets you get Pokémon names.

## Installation

`cargo install pokemon`


## API

### Notes

Currently, Pokémon are limited to the following properties:


| Property | Type           |
|----------|----------------|
| species  | `i32`           |
| language | `i32`          |
| name     | `String`         |
| genus    | `Option<String>` |

Languages correspond to these ids:

| id | language                  |
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

### `pokemon::get_all()`

Returns a vector of `Pokemon`.

### `pokemon::get_pokemon(id, lang)`

Returns a Pokémon based on the specified id and language.

### `pokemon::get_random()`

Returns a random Pokémon.

### `pokemon::get_random_with_lang()`

Returns a random Pokémon, with a specified language.

### `pokemon::get_name(id)`

Returns the name of a Pokémon based on its id. (In English).

### `pokemon::get_name_with_lang(id, lang)`

Returns the name of a Pokémon, given id and language.
