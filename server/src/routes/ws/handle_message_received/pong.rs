use crate::Connections;

pub async fn pong(user_id: &str, connections: &Connections) {
    let connections = connections.read().await;
    let connection_data = if let Some(connection_data) = connections.get(user_id) {
        connection_data
    } else {
        eprintln!("Could not find connection_data for user {}", user_id);
        return;
    };

    let mut is_alive = connection_data.is_alive.write().await;
    *is_alive = true;
}
