use warp::{http::StatusCode, Reply, reply};
use crate::{ItemsDb, models::models, Result};

pub async fn hello(name: String) -> Result<impl warp::Reply>{
    let reply= format!("Holo, {}!", name);
    println!("{}", &reply);
    Ok(warp::reply::html(reply))
}

pub async fn get_shopping_list_items(items_db: ItemsDb) -> Result<impl Reply> {
    let local_db = items_db.lock().await;
    let local_db: Vec<models::ShoppingListItem> = local_db.values().cloned().collect();
    Ok(reply::with_status(reply::json(&local_db), StatusCode::OK))
}

pub async fn creating_shopping_list_item(mut shopping_list_item: models::ShoppingListItem, items_db: ItemsDb,) -> Result<impl Reply> {
    println!("Received UserData: {:?}", shopping_list_item);
    let mut local_db = items_db.lock().await;
    let key_count = local_db.keys().len();
    shopping_list_item.item_id = Some(key_count);
    local_db.insert(key_count, shopping_list_item.clone());

    Ok(reply::with_status(reply::json(&shopping_list_item), StatusCode::CREATED)) 
}