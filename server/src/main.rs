// #![deny(warnings)]
extern crate common;
mod handlers;

use handlers::{index, ws};
use warp::Filter;
use common::Game;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use uuid::Uuid;
use warp::ws::Message;

pub type Websockets = Arc<RwLock<HashMap<String, mpsc::UnboundedSender<Message>>>>;
pub type Games = Arc<RwLock<HashMap<String, Game>>>;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    // universal app state
    let users = Websockets::default();
    let games = Games::default();

    // GET /ws -> websocket upgrade
    let ws_route = warp::path("ws")
        // The `ws()` filter will prepare Websocket handshake...
        .and(warp::ws())
        // get `user_id` query parameter
        .and(warp::filters::query::raw().map(|e: String| {
            let result = e.split_once('=').expect("Couldn't split string at '='");
            String::from(result.1)
        }))
        // get users hashmap
        .and(warp::any().map(move || users.clone()))
        // get games hashmap
        .and(warp::any().map(move || games.clone()))
        // combine filters into a handler function
        .map(|ws: warp::ws::Ws, user_id: String, users, games| {
            eprint!("Query parameter: user_id = {:#?}\n", &user_id);
            // This will call our function if the handshake succeeds.
            ws.on_upgrade(move |socket| ws::handle_ws_upgrade(socket, user_id, users, games))
        });

    // GET / -> index html
    let index_route = warp::path::end().map(|| warp::reply::html(index::INDEX_HTML));

    let routes = index_route.or(ws_route);

    warp::serve(routes).run(([127, 0, 0, 1], 8001)).await;
}
