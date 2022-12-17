// #![deny(warnings)]
extern crate common;
mod errors;
mod routes;

use common::{PrivateGameState, STCMsg, NO_USER_ID};
use futures::join;
use routes::{
    index,
    ws::{self, send_ws_message},
};
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{
    mpsc::{self, error::SendError},
    RwLock,
};
use tokio::{task, time};
use warp::ws::Message;
use warp::Filter;

/// Maps `user_id`s to websocket connections and `game_codes`
pub type Connections = Arc<RwLock<HashMap<String, ConnectionData>>>;

/// Maps `game_id`s to game states
pub type Games = Arc<RwLock<HashMap<String, PrivateGameState>>>;

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

    // global app state
    let connections = Connections::default();
    let games = Games::default();
    let game_codes = GameCodes::default();

    let connections_clone = Arc::clone(&connections);

    // send ping messages every 5 messages to every websocket
    let ping_pong = task::spawn(async move {
        let mut interval = time::interval(Duration::from_millis(PING_INTERVAL_MS));
        loop {
            interval.tick().await;
            for (user_id, connection_data) in connections_clone.read().await.iter() {
                if connection_data.connected && !*connection_data.is_alive.read().await {
                    // user is still connected but didn't respond to ping: close their websocket
                    eprintln!("Closing websocket connection for idle user {}", &user_id);

                    let result = connection_data.tx.send(Message::close());
                    if let Err(SendError(message)) = result {
                        eprintln!(
                            "Couldn't send internal websocket message {:?}",
                            message.to_str()
                        );
                    }
                } else {
                    // send ping to user
                    let mut is_alive = connection_data.is_alive.write().await;
                    *is_alive = false;
                    send_ws_message::to_user(
                        user_id,
                        STCMsg::Ping,
                        &Arc::clone(&connections_clone),
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
            let user_id = if let Some(user_id) = e.split_once('=') {
                user_id.1
            } else {
                eprintln!("Error parsing user_id from query parameters. Using NO_USER_ID value instead for user");
                NO_USER_ID
            };
            String::from(user_id)
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

    // docker provides PORT=8080
    let port: u16 = if let Ok(port) = env::var("PORT") {
        port
    } else {
        "8080".to_string()
    }
    .parse()
    .expect("Could not parse provided PORT environment variable into u16");

    let (_, _) = join!(warp::serve(routes).run(([0, 0, 0, 0], port)), ping_pong);
}
