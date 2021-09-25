use crate::{errors::USER_ID_NOT_IN_MAP, Connections};

pub async fn pong(user_id: &str, connections: &Connections) {
    let connections = connections.read().await;
    let ws = connections.get(user_id).expect(USER_ID_NOT_IN_MAP);
    let mut is_alive = ws.is_alive.write().await;
    *is_alive = true;
}
