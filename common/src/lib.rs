#![feature(format_args_capture)]
extern crate rand;
mod cards;
mod consts;
mod game_stage;
mod game_state;
mod messages;
mod teams;
mod tichu;
mod trade;
mod user;
mod utils;

pub use cards::*;
pub use consts::*;
pub use game_stage::*;
pub use game_state::*;
pub use messages::*;
pub use teams::*;
pub use tichu::*;
pub use trade::*;
pub use user::*;

pub use utils::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
