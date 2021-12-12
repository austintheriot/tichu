#![feature(format_args_capture)]
#[cfg(feature = "client")]
extern crate js_sys;
extern crate rand;
mod cards;
mod game_stage;
mod game_state;
mod messages;
mod play;
mod teams;
mod tichus;
mod trade;
mod user;
mod utils;

pub use cards::*;
pub use game_stage::*;
pub use game_state::*;
pub use messages::*;
pub use play::*;
pub use teams::*;
pub use tichus::*;
pub use trade::*;
pub use user::*;
pub use utils::*;

#[cfg(all(feature = "client", feature = "server"))]
compile_error!("feature \"client\" and feature \"server\" cannot be enabled at the same time");
