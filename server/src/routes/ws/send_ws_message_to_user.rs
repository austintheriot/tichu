use common::STCMsg;
use warp::ws::Message;

use crate::{errors::USER_ID_NOT_IN_MAP, Connections};

/// Sends a single server-to-client websocket message to a single participant.
pub async fn send_ws_message_to_user(user_id: &str, msg: STCMsg, connections: &Connections) {
    let msg = bincode::serialize(&msg).expect("Could not serialize message");
    let msg = Message::binary(msg);
    let read_connections = connections.read().await;
    let ws = read_connections.get(user_id).expect(USER_ID_NOT_IN_MAP);
    if let Err(_disconnected) = ws.tx.send(msg.clone()) {
        eprint!("User is disconnected. Couldn't send message {:?}\n", &msg);
    } else {
        eprint!("Message successfully sent\n");
    }
}
