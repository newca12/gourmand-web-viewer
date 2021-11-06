extern crate serde;
extern crate serde_derive;

pub mod recipe;

use std::collections::HashMap;
use std::collections::HashSet;

use recipe::Recipe;

pub fn load(debug: bool) -> HashMap<String, Recipe> {
    let mut recipes = HashMap::new();
    let mut categories = HashSet::new();
    let mut cuisines = HashSet::new();

    let recipes_data = include_str!("data/recipes.xml");
    let result = recipe::read_from_str(recipes_data).unwrap();
    for recipe in result.recipe {
        categories.insert(recipe.category.clone());
        cuisines.insert(recipe.cuisine.clone());
        if recipe.ingredient_list.is_some() {
            let title = recipe.title.clone();
            let r = recipes.insert(recipe.title.clone(), recipe);
            if debug && r.is_some() {
                println!("Duplicate entry detected for {}", title);
            }
        } else if debug {
            println!("Recipe without ingredient for {}", recipe.title);
        }
    }
    //println!("{:?}", recipes);
    //println!("{:?}", categories);
    //println!("{:?}", cuisines);
    recipes
}

pub fn check(recipes: HashMap<String, Recipe>) {
    for (key, value) in recipes.into_iter() {
        for ingredient in value.ingredient_list.unwrap().ingredients {
            if ingredient.item.is_some() {
                let item = ingredient.item.unwrap();
                if ingredient.key != item {
                    println!(
                        "key & Item does not match in recipe  {} : {} / {}",
                        key, ingredient.key, item
                    );
                }
            } else {
                println!("Missing item in recipe {}", key);
            }
        }
    }
}
