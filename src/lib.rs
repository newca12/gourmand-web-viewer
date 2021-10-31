#[macro_use]
extern crate serde_derive;
extern crate serde;

pub mod recipe;

use std::collections::HashMap;

use recipe::Ingredient;

#[derive(Serialize, Deserialize)]
struct Recipe {
    name: String,
    items: Vec<String>,
}

pub fn load() -> HashMap<String, Vec<Ingredient>> {
    let mut recipes = HashMap::new();
    let recipes_data = include_str!("data/recipes.xml");
    let result = recipe::read_from_str(recipes_data).unwrap();
    for recipe in result.recipe {
        if recipe.ingredient_list.is_some() {
            recipes.insert(recipe.title, recipe.ingredient_list.unwrap().ingredients);
        } //else { println!("{}", recipe.title); }
    }
    recipes
}
