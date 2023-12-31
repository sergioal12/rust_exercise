use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]

pub enum ShoppingListItemType {
    Drink,
    Desert,
    Fruit,
    Snack,
    Spread,
    Vegetable,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShoppingListItem{
    pub item_id: Option<usize>,
    pub name: String,
    pub item_item: ShoppingListItemType,
    pub description: String,
    pub price: f32
}