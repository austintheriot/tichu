// #![deny(warnings)]
extern crate common;
mod handlers;

use common::{Game, STCMsg};
use futures::join;
use handlers::{
    index,
    ws::{self, send_message},
};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{mpsc, RwLock};
use tokio::{task, time}; // 1.3.0
use warp::ws::Message;
use warp::Filter;

pub type Websockets = Arc<RwLock<HashMap<String, mpsc::UnboundedSender<Message>>>>;
pub type Games = Arc<RwLock<HashMap<String, Game>>>;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    // universal app state
    let users = Websockets::default();
    let games = Games::default();

    // send ping messages every 5 messages to every websocket
    let cloned_users = users.clone();
    let cloned_games = games.clone();
    let ping_websockets = task::spawn(async move {
        let mut interval = time::interval(Duration::from_millis(5000));
        loop {
            interval.tick().await;
            for (user_id, _) in cloned_users.read().await.iter() {
                send_message(
                    user_id.to_string(),
                    STCMsg::Ping,
                    &cloned_users.clone(),
                    &cloned_games.clone(),
                )
                .await;
            }
        }
    });

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
            // This will call our function if the handshake succeeds.
            ws.on_upgrade(move |socket| ws::handle_ws_upgrade(socket, user_id, users, games))
        });

    // GET / -> index html
    let index_route = warp::path::end().map(|| warp::reply::html(index::INDEX_HTML));

    let routes = index_route.or(ws_route);

    join!(
        warp::serve(routes).run(([127, 0, 0, 1], 8001)),
        ping_websockets
    );
}
