#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]
//! documentation for pokemon
//! Yep.
extern crate csv;
extern crate rand;
extern crate serde;

pub mod pokemon {
    //! documentation for pokemon
    //! Yep.
    use csv;
    use rand::{thread_rng, Rng};
    use serde::{Deserialize, Serialize};
    use std::fs::File;
    use std::path::Path;

    /// The struct for `Pokemon`
    #[derive(Deserialize, Serialize, Clone, Debug)]
    pub struct Pokemon {
        species: i32,
        language: i32,
        name: String,
        genus: Option<String>,
    }

    fn search<P: AsRef<Path>>(file_path: P) -> Vec<Pokemon> {
        let file = File::open(file_path).unwrap();
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(file);
        rdr.deserialize::<Pokemon>()
            .collect::<Result<Vec<Pokemon>, csv::Error>>()
            .unwrap()
    }

    fn search_one<P: AsRef<Path>>(file_path: P, index: usize, lang_id: i32) -> Pokemon {
        let file = File::open(file_path).unwrap();
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(file);
        let rows: Vec<Pokemon> = rdr
            .deserialize::<Pokemon>()
            .collect::<Result<Vec<Pokemon>, csv::Error>>()
            .unwrap();

        let mut pokey: Pokemon = Pokemon {
            species: 1,
            language: 1,
            name: "Bulbasaur".to_string(),
            genus: Some("seed".to_string()),
        };

        for poke in &rows {
            if poke.species == index as i32 && poke.language == lang_id {
                //println!("{:?}", poke);
                pokey = poke.clone();
            }
        }

        pokey
    }

    /// Returns all the Pokemon, in each language
    #[allow(dead_code)]
    pub fn get_all() -> Vec<Pokemon> {
        search("data/pokemon.csv")
    }

    /// Returns a single Pokemon based on specified id and language
    /// MB: English is 9
    #[allow(dead_code)]
    pub fn get_pokemon(id: usize, lang_id: i32) -> Pokemon {
        search_one("data/pokemon.csv", id, lang_id)
    }

    /// Returns a random Pokemon in English
    #[allow(dead_code)]
    pub fn get_random() -> Pokemon {
        let mut rng = thread_rng();
        search_one("data/pokemon.csv", rng.gen_range(1..=1025), 9)
    }

    /// Returns a random pokemon with the specified language
    #[allow(dead_code)]
    pub fn get_random_with_lang(lang_id: i32) -> Pokemon {
        let mut rng = thread_rng();
        search_one("data/pokemon.csv", rng.gen_range(1..=1025), lang_id)
    }

    /// Returns the name of the specified id
    /// Using 9 for example, would return "Blastoise"
    #[allow(dead_code, unused_variables)]
    pub fn get_name(id: usize) -> String {
        search_one("data/pokemon.csv", id, 9).name
    }

    /// Same as get_name(), but allows you to specify the language
    #[allow(dead_code)]
    pub fn get_name_with_lang(id: usize, lang_id: i32) -> String {
        search_one("data/pokemon.csv", id, lang_id).name
    }
}
