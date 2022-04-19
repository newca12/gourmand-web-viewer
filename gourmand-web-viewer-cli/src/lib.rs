extern crate serde;
extern crate serde_derive;

use gourmand_web_viewer::recipe;
//pub mod recipe;

use std::collections::HashMap;
use std::collections::HashSet;

use include_flate::flate;
use recipe::Recipe;

pub fn load_json() -> (HashSet<String>, HashSet<String>, HashMap<String, Recipe>) {
    flate!(static RECIPES: str from "../gourmand-web-viewer/src/data/recipes.json");
    flate!(static CATEGORIES: str from "../gourmand-web-viewer/src/data/categories.json");
    flate!(static CUISINES: str from "../gourmand-web-viewer/src/data/cuisines.json");
    let recipes = serde_json::from_str(&RECIPES).unwrap();
    let categories = serde_json::from_str(&CATEGORIES).unwrap();
    let cuisines = serde_json::from_str(&CUISINES).unwrap();
    (categories, cuisines, recipes)
}

pub fn load_xml(debug: bool) -> (HashSet<String>, HashSet<String>, HashMap<String, Recipe>) {
    let mut recipes = HashMap::new();
    let mut categories = HashSet::new();
    let mut cuisines = HashSet::new();

    flate!(static RECIPES_DATA: str from "src/data/recipes.xml");
    let result = recipe::read_from_str(&RECIPES_DATA).unwrap();
    for recipe in result.recipe {
        let title = recipe.title.clone();
        if recipe.category.is_some() {
            categories.insert(recipe.category.clone().unwrap());
        } else if debug && recipe.category.is_none() {
            println!("No category for {}", title);
        }
        if recipe.cuisine.is_some() {
            cuisines.insert(recipe.cuisine.clone().unwrap());
        } else if debug && recipe.cuisine.is_none() {
            println!("No cuisine for {}", title);
        }
        if recipe.ingredient_list.is_some() {
            let mut ok = true;
            for ingredient in recipe.clone().ingredient_list.unwrap().ingredients {
                if ingredient.key.is_none() {
                    ok = false;
                }
            }
            if ok {
                let r = recipes.insert(recipe.title.clone(), recipe);
                if debug && r.is_some() {
                    println!("DISCARDED : Duplicate entry detected for {}", title);
                }
            } else if debug {
                println!(
                    "DISCARDED : Recipe with ingredients with no key for {}",
                    recipe.title
                );
            }
        } else if debug {
            println!("DISCARDED : Recipe without ingredient for {}", recipe.title);
        }
    }
    if debug {
        //println!("{:?}", recipes);
        println!("{:?}", categories);
        println!("{:?}", cuisines);
    }
    (categories, cuisines, recipes)
}

pub fn check(recipes: HashMap<String, Recipe>) -> bool {
    let mut ok = true;
    for (key, value) in recipes.into_iter() {
        for ingredient in value.ingredient_list.unwrap().ingredients {
            if ingredient.key.is_none() {
                println!("Missing key in recipe {}", key);
                ok = false
            }
            if ingredient.item.is_some() {
                let item = ingredient.item.unwrap();
                let key_ing = ingredient.key.unwrap();
                if key_ing != item {
                    println!(
                        "key & Item does not match in recipe  {} : {} / {}",
                        key, key_ing, item
                    );
                }
            } else {
                println!("Missing item in recipe {}", key);
            }
        }
    }
    ok
}
