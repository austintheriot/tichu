// #![deny(warnings)]
#![feature(never_type)]
extern crate common;
mod errors;
mod handlers;

use common::{GameState, STCMsg};
use futures::join;
use handlers::{
    index,
    ws::{self, send_ws_message_to_user},
};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{mpsc, RwLock};
use tokio::{task, time};
use warp::ws::Message;
use warp::Filter;

use crate::handlers::ws::CLOSE_WEBSOCKET;

/// Maps `user_id`s to websocket connections and `game_codes`
pub type Connections = Arc<RwLock<HashMap<String, ConnectionData>>>;

/// Maps `game_id`s to game states
pub type Games = Arc<RwLock<HashMap<String, GameState>>>;

/// Maps 4-character `game_code`s -> `game_id`s
pub type GameCodes = Arc<RwLock<HashMap<String, String>>>;

#[derive(Debug)]
pub struct ConnectionData {
    pub user_id: String,
    pub game_id: Option<String>,
    /// Used for ping/pong diagnostics
    pub is_alive: Arc<RwLock<bool>>,
    /// Is the user's websocket currently connected?
    pub connected: bool,
    /// Channel for sending messages through the websocket
    pub tx: mpsc::UnboundedSender<Message>,
}

static PING_INTERVAL_MS: u64 = 5_000;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    // universal app state
    let connections = Connections::default();
    let games = Games::default();
    let game_codes = GameCodes::default();

    let connections_clone = Arc::clone(&connections);
    let games_clone = Arc::clone(&games);
    let game_codes_clone = Arc::clone(&game_codes);

    // send ping messages every 5 messages to every websocket
    let ping_pong = task::spawn(async move {
        let mut interval = time::interval(Duration::from_millis(PING_INTERVAL_MS));
        loop {
            interval.tick().await;
            for (user_id, ws) in connections_clone.read().await.iter() {
                if !*ws.is_alive.read().await {
                    // user didn't respond to ping, close their websocket
                    eprint!("Closing websocket connection for idle user {}", &user_id);
                    ws.tx
                        .send(Message::text(CLOSE_WEBSOCKET))
                        .expect("Couldn't send internal CLOSE websocket message");
                } else {
                    // send ping to user
                    let mut is_alive = ws.is_alive.write().await;
                    *is_alive = false;
                    send_ws_message_to_user(
                        user_id.into(),
                        STCMsg::Ping,
                        &Arc::clone(&connections_clone),
                        &Arc::clone(&games_clone),
                        &Arc::clone(&game_codes_clone),
                    )
                    .await;
                }
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
        // get connections hashmap
        .and(warp::any().map(move || Arc::clone(&connections)))
        // get games hashmap
        .and(warp::any().map(move || Arc::clone(&games)))
        // get game codes hashmap
        .and(warp::any().map(move || Arc::clone(&game_codes)))
        // combine filters into a handler function
        .map(
            |ws: warp::ws::Ws, user_id: String, connections, games, game_codes| {
                // This will call our function if the handshake succeeds.
                ws.on_upgrade(move |socket| {
                    ws::handle_ws_upgrade(socket, user_id, connections, games, game_codes)
                })
            },
        );

    // GET / -> index html
    let index_route = warp::path::end().map(|| warp::reply::html(index::INDEX_HTML));

    let routes = index_route.or(ws_route);

    let (_, _) = join!(warp::serve(routes).run(([127, 0, 0, 1], 8001)), ping_pong);
}
