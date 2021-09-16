// #![deny(warnings)]

mod index;
mod state;
mod ws;

extern crate common;
use state::{Games, Websockets};
use warp::Filter;

#[derive(Debug)]
struct Test {
    user_id: String,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    // Keep track of all connected users, key is usize, value
    // is a websocket sender.
    let users = Websockets::default();
    let games = Games::default();

    // GET /ws -> websocket upgrade
    let ws_route = warp::path("ws")
        // The `ws()` filter will prepare Websocket handshake...
        .and(warp::ws())
        // get user_id query parameter
        .and(warp::filters::query::raw().map(|e: String| {
            let result = e.split_once('=').expect("Couldn't split string at '='");
            String::from(result.1)
        }))
        .and(warp::any().map(move || users.clone()))
        .and(warp::any().map(move || games.clone()))
        .map(|ws: warp::ws::Ws, query_parameters: String, users, games| {
            eprint!("Query parameter: user_id = {:#?}\n", &query_parameters);
            // This will call our function if the handshake succeeds.
            ws.on_upgrade(move |socket| ws::handle_ws_upgrade(socket, users, games))
        });

    // GET / -> index html
    let index_route = warp::path::end().map(|| warp::reply::html(index::INDEX_HTML));

    let routes = index_route.or(ws_route);

    warp::serve(routes).run(([127, 0, 0, 1], 8001)).await;
}
