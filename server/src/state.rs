extern crate common;

use common::Game;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use uuid::Uuid;
use warp::ws::Message;

/// Our state of currently connected users.
///
/// - Key is their id
/// - Value is a sender of `warp::ws::Message`
pub type Websockets = Arc<RwLock<HashMap<Uuid, mpsc::UnboundedSender<Message>>>>;

pub type Games = Arc<RwLock<HashMap<Uuid, Game>>>;
