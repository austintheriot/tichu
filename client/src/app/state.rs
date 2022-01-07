use anyhow::Error;
use common::{CallGrandTichuRequest, Card, CardValue, OtherPlayerOption, PublicGameState};
use log::*;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use wasm_bindgen::prelude::Closure;
use web_sys::{ErrorEvent, MessageEvent};
use yew::{Reducible, UseReducerHandle};

pub const USER_ID_STORAGE_KEY: &str = "yew.tichu.user_id";
pub const DISPLAY_NAME_STORAGE_KEY: &str = "yew.tichu.display_name";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum WSConnectionStatus {
    Open,
    Error,
    Closed,
}

pub struct WSCallbacks {
    #[allow(dead_code)]
    onmessage: Closure<dyn FnMut(MessageEvent)>,
    #[allow(dead_code)]
    onopen: Closure<dyn FnMut()>,
    #[allow(dead_code)]
    onerror: Closure<dyn FnMut(ErrorEvent)>,
    #[allow(dead_code)]
    onclose: Closure<dyn FnMut()>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppState {
    ws_connection_status: WSConnectionStatus,
    user_id: String,
    display_name: String,
    game_state: Option<PublicGameState>,
    is_alive: bool,

    join_room_game_code_input: String,
    display_name_input: String,
    team_a_name_input: String,
    team_b_name_input: String,

    /// temporary card selected for trading, etc. (i.e. game stage is not Play)
    /// this card is then moved into `trade_to_opponent` state once the user chooses
    /// who to give the card to
    selected_pre_play_card: Option<Card>,
    trade_to_opponent1: Option<Card>,
    trade_to_teammate: Option<Card>,
    trade_to_opponent2: Option<Card>,

    /// cards selected for playing
    selected_play_cards: Vec<Card>,
    wished_for_card_value: Option<CardValue>,
    user_id_to_give_dragon_to: Option<String>,
    show_user_id_to_give_dragon_to_form: bool,
}

impl Reducible for AppState {
    /// Reducer Action Type
    type Action = AppReducerAction;

    /// Reducer Function
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut next_state = (*self).clone();
        match action {
            AppReducerAction::Debug => {
                log!("Debug action called");
            }
            _ => {}
        }
        Rc::new(next_state)
    }
}

pub enum AppReducerAction {
    ConnectToWS,
    BeginPing,
    WebsocketOpen,
    WebsocketError,
    WebsocketClosed,
    WSMsgReceived(Result<Vec<u8>, Error>),
    SendWSMsg(CTSMsgInternal),
    SetUserId(String),
    SetDisplayName(String),
    SetDisplayNameInput(String),
    SetJoinRoomGameCodeInput(String),
    SetTeamANameInput(String),
    SetTeamBNameInput(String),
    SetSelectedPrePlayCard(usize),
    RemoveSelectedPrePlayCard,

    SetTrade(OtherPlayerOption),
    RemoveTrade(OtherPlayerOption),
    AddSelectedPlayCard(usize),
    RemoveSelectedPlayCard(usize),
    SetUserIdToGiveDragonTo(Option<String>),
    SetWishedForCard(usize),
    Debug,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AppContext {
    pub reducer_handle: UseReducerHandle<AppState>,
}

/// Internal Tichu-client message for alerting that it's time to send a websocket message
///
/// This type reflects the common::CTSMsg, except with all data values tripped,
/// since the data values are formulated in the send_ws_message message
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum CTSMsgInternal {
    JoinGameWithGameCode,
    MoveToTeam(TeamOption),
    RenameTeam(TeamOption),
    CreateGame,
    LeaveGame,
    StartGrandTichu,
    SubmitTrade,
    PlayCards,
    Pass,
    CallGrandTichu(CallGrandTichuRequest),
    CallSmallTichu,
    __Admin_SkipToPlay,

    Ping,
    Pong,
    Test,
}
