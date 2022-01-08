use crate::global::state::AppContext;
use crate::global::state::AppState;
use crate::global::ws::use_setup_app_ws;
use crate::ui::debug::debug::Debug;
use crate::ui::join::join::Join;
use crate::ui::lobby::lobby::Lobby;
use crate::ui::teams::teams::Teams;
use common::PublicGameStage;
use yew::{function_component, html, prelude::*};

#[function_component(App)]
pub fn app() -> Html {
    // initialize app state
    let app_reducer_handle = use_reducer_eq(AppState::default);

    // setup websocket and begin pinging server
    let send_ws_message = use_setup_app_ws(app_reducer_handle.clone());

    // pass app state and a send_ws_message utility down through context
    let context = AppContext {
        app_reducer_handle: app_reducer_handle.clone(),
        send_ws_message,
    };

    let app_state = &*app_reducer_handle;

    html! {
        <ContextProvider<AppContext> {context}>
            <div>
            {match &app_state.game_state {
                None => html! { <Join /> },
                Some(game_state) =>{
                    match game_state.stage {
                        PublicGameStage::Lobby => html! { <Lobby /> },
                        PublicGameStage::Teams(_) => html!{ <Teams /> },
                        PublicGameStage::GrandTichu(_) => todo!(),
                        PublicGameStage::Trade(_) => todo!(),
                        PublicGameStage::Play(_) => todo!(),
                        _ => html!{<> </>}
                    }
                }
            }}
            <br />
            <br />
            <hr />
            <br />
            <Debug />
            </div>
        </ContextProvider<AppContext>>
    }
}
