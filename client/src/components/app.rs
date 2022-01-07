use super::lobby::Lobby;
use crate::app::state::{
    AppContext, AppState, WSConnectionStatus, DISPLAY_NAME_STORAGE_KEY, USER_ID_STORAGE_KEY,
};
use common::NO_USER_ID;
use gloo::storage::{LocalStorage, Storage};
use yew::{function_component, html, prelude::*};

#[function_component(App)]
pub fn app() -> Html {
    let reducer_handle = use_reducer_eq(|| {
        // retrieve user_id and display name from local storage
        let user_id =
            LocalStorage::get(USER_ID_STORAGE_KEY).unwrap_or_else(|_| String::from(NO_USER_ID));
        let display_name =
            LocalStorage::get(DISPLAY_NAME_STORAGE_KEY).unwrap_or_else(|_| String::from(""));

        // store user_id and display_name in local storage (if changed)
        LocalStorage::set(USER_ID_STORAGE_KEY, &user_id)
            .expect("failed to save user_id to local storage");
        LocalStorage::set(DISPLAY_NAME_STORAGE_KEY, &display_name)
            .expect("failed to save display_name to local storage");

        AppState {
            ws_connection_status: WSConnectionStatus::Closed,
            user_id,
            display_name: display_name.clone(),
            display_name_input: display_name,
            game_state: None,
            join_room_game_code_input: "".into(),
            is_alive: false,
            team_a_name_input: "".into(),
            team_b_name_input: "".into(),
            selected_pre_play_card: None,
            trade_to_opponent1: None,
            trade_to_teammate: None,
            trade_to_opponent2: None,
            selected_play_cards: Vec::new(),
            user_id_to_give_dragon_to: None,
            show_user_id_to_give_dragon_to_form: false,
            wished_for_card_value: None,
        }
    });

    let context = AppContext {
        reducer_handle: reducer_handle.clone(),
    };

    html! {
        <ContextProvider<AppContext> {context}>
            <Lobby />
        </ContextProvider<AppContext>>
    }
}
