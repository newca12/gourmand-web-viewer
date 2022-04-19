extern crate serde;
extern crate serde_derive;

pub mod recipe;

use std::collections::HashMap;
use std::collections::HashSet;

use include_flate::flate;
use recipe::Recipe;

pub fn load_json() -> (HashSet<String>, HashSet<String>, HashMap<String, Recipe>) {
    flate!(static RECIPES: str from "src/data/recipes.json");
    flate!(static CATEGORIES: str from "src/data/categories.json");
    flate!(static CUISINES: str from "src/data/cuisines.json");
    let recipes = serde_json::from_str(&RECIPES).unwrap();
    let categories = serde_json::from_str(&CATEGORIES).unwrap();
    let cuisines = serde_json::from_str(&CUISINES).unwrap();
    (categories, cuisines, recipes)
}
