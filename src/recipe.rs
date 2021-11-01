use serde_derive::{Deserialize, Serialize};
use serde_xml_rs;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct GourmetDoc {
    #[serde(rename = "recipe")]
    pub recipe: Vec<Recipe>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Recipe {
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "ingredient-list")]
    pub ingredient_list: Option<IngredientList>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IngredientList {
    #[serde(rename = "ingredient")]
    pub ingredients: Vec<Ingredient>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Ingredient {
    #[serde(rename = "item")]
    pub item: Option<String>,
    #[serde(rename = "key")]
    pub key: String,
}

pub fn read_from_str(str: &str) -> Result<GourmetDoc, serde_xml_rs::Error> {
    serde_xml_rs::from_str(str)
}
