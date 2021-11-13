extern crate serde;
extern crate serde_derive;

pub mod recipe;

use std::collections::HashMap;
use std::collections::HashSet;

use recipe::Recipe;

pub fn load(debug: bool) -> (HashSet<String>, HashSet<String>, HashMap<String, Recipe>) {
    let mut recipes = HashMap::new();
    let mut categories = HashSet::new();
    let mut cuisines = HashSet::new();

    let recipes_data = include_str!("data/recipes.xml");
    let result = recipe::read_from_str(recipes_data).unwrap();
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
