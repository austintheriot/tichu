use common::STCMsg;

use crate::{routes::ws::send_ws_message, Connections};

pub async fn test(user_id: &str, connections: &Connections) {
    send_ws_message::to_user(user_id, STCMsg::Test("Hello client!".into()), connections).await;
}
