use warp::{self, Filter, Rejection};// 1.
use std::{collections::HashMap, convert::Infallible, sync::Arc};
use crate::models::models::ShoppingListItem;
use tokio::sync::Mutex;
use console::Style;

mod routes;
mod handlers;
mod api;
mod models;

use self::{
    routes::hello_route,
    handlers::hello_handlers
};

type ItemsDb = Arc<Mutex<HashMap<usize, ShoppingListItem>>>;
type Result<T> = std::result::Result<T, Rejection>;

#[cfg(test)] mod test;

#[tokio::main] // 2.
async fn main() {
    let items_db: ItemsDb = Arc::new(Mutex::new(HashMap::new()));

   let target: String = "0.0.0.0:8080".parse().unwrap();
   let blue = Style::new().blue();

   let end = hello!().with(warp::log("holo!!"));
   println!("RustWarp ready at {}", blue.apply_to(&target));
   println!("use $curl 0.0.0.0:8000/hello/loquesea to test the endpoint.");
   let root = warp::path::end().map(||"warp server started");
   let shopping_list_items_route = warp::path("shopping_list_items")
                .and(warp::get())
                .and(with_items_db(items_db.clone()))
                .and_then(hello_handlers::get_shopping_list_items);
    let create_shopping_list_items_route = warp::path("create_shopping_items")
                .and(warp::post())
                .and(warp::body::json())
                .and(with_items_db(items_db.clone()))
                .and_then(hello_handlers::creating_shopping_list_item);
   let routes = root
   .or(shopping_list_items_route)
   .or(create_shopping_list_items_route)
   .with(warp::cors().allow_any_origin());


   warp::serve(routes).run(([0,0,0,0], 8000)).await;
}

fn with_items_db(items_db: ItemsDb, ) -> impl Filter<Extract=(ItemsDb,), Error= Infallible> + Clone {
    warp::any().map(move|| items_db.clone())
}
