use anyhow::Error;
use common::{
    get_card_combination, get_user_can_play_wished_for_card, next_combo_beats_prev,
    validate_display_name, validate_game_code, CallGrandTichuRequest, Card, CardValue,
    OtherPlayerOption, PublicGameStage, PublicGameState, TeamCategories, TeamOption,
    TichuCallStatus, ValidCardCombo, DRAGON, NO_USER_ID,
};
use gloo::storage::{LocalStorage, Storage};
use log::*;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use wasm_bindgen::prelude::Closure;
use web_sys::{ErrorEvent, MessageEvent};
use yew::{use_reducer_eq, Reducible, UseReducerHandle};

pub const USER_ID_STORAGE_KEY: &str = "yew.tichu.user_id";
pub const DISPLAY_NAME_STORAGE_KEY: &str = "yew.tichu.display_name";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum WSConnectionStatus {
    Open,
    Error,
    Closed,
}

pub enum AppReducerAction {
    ConnectToWS,
    BeginPing,
    SetIsAlive(bool),
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
    ResetAfterPlayCards,

    SetTrade(OtherPlayerOption),
    RemoveTrade(OtherPlayerOption),
    AddSelectedPlayCard(usize),
    RemoveSelectedPlayCard(usize),
    SetUserIdToGiveDragonTo(Option<String>),
    SetShowUserIdToGiveDragonToForm(bool),
    SetWishedForCard(usize),
    Debug,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppState {
    pub ws_connection_status: WSConnectionStatus,
    pub user_id: String,
    pub display_name: String,
    pub game_state: Option<PublicGameState>,
    pub is_alive: bool,

    pub join_room_game_code_input: String,
    pub display_name_input: String,
    pub team_a_name_input: String,
    pub team_b_name_input: String,

    /// temporary card selected for trading, etc. (i.e. game stage is not Play)
    /// this card is then moved into `trade_to_opponent` state once the user chooses
    /// who to give the card to
    pub selected_pre_play_card: Option<Card>,
    pub trade_to_opponent1: Option<Card>,
    pub trade_to_teammate: Option<Card>,
    pub trade_to_opponent2: Option<Card>,

    /// cards selected for playing
    pub selected_play_cards: Vec<Card>,
    pub wished_for_card_value: Option<CardValue>,
    pub user_id_to_give_dragon_to: Option<String>,
    pub show_user_id_to_give_dragon_to_form: bool,
}

impl Reducible for AppState {
    /// Reducer Action Type
    type Action = AppReducerAction;

    /// Reducer Function
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut next_state = (*self).clone();
        match action {
            AppReducerAction::Debug => {
                info!("Debug action called");
            }
            AppReducerAction::ResetAfterPlayCards => {
                next_state.selected_play_cards.drain(..);
                next_state.user_id_to_give_dragon_to = None;
                next_state.wished_for_card_value = None;
            }
            AppReducerAction::SetShowUserIdToGiveDragonToForm(bool) => {
                next_state.show_user_id_to_give_dragon_to_form = bool;
            }
            _ => {}
        }
        Rc::new(next_state)
    }
}

impl AppState {
    pub fn get_prev_played_combo(&self) -> Option<&ValidCardCombo> {
        return if let Some(game_state) = &self.game_state {
            if let PublicGameStage::Play(play_stage) = &game_state.stage {
                play_stage.table.last()
            } else {
                None
            }
        } else {
            None
        };
    }

    pub fn can_play_cards(&self) -> bool {
        let game_state = if let Some(game_state) = &self.game_state {
            game_state
        } else {
            return false;
        };

        // must be users turn OR must be playable bomb
        let combo = get_card_combination(
            self.get_prev_played_combo(),
            &self.selected_play_cards,
            &self.user_id,
        );

        let combo_contains_dragon = self.selected_play_cards.contains(&DRAGON);
        let user_has_chosen_a_user_to_given_dragon_to = self.user_id_to_give_dragon_to.is_some();

        let wished_for_card_value = game_state.get_wished_for_card_value();

        let some_card_has_been_wished_for = wished_for_card_value.is_some();

        if let Some(combo) = combo {
            let user_can_play_wished_for_card = if some_card_has_been_wished_for {
                let wished_for_card_value = wished_for_card_value.as_ref().unwrap();
                get_user_can_play_wished_for_card(
                    self.get_prev_played_combo(),
                    &game_state.current_user.hand,
                    wished_for_card_value,
                )
            } else {
                false
            };
            let combo_contains_wished_for_card = if some_card_has_been_wished_for {
                let wished_for_card_value = wished_for_card_value.as_ref().unwrap();
                combo
                    .cards()
                    .iter()
                    .any(|card| card.value == *wished_for_card_value)
            } else {
                false
            };

            self.stage_is_play()
                && (self.is_current_users_turn() || combo.is_bomb())
                && self.hand_beats_combo_on_table(&combo)
                && (!combo_contains_dragon
                    || combo_contains_dragon && user_has_chosen_a_user_to_given_dragon_to)
                && (!some_card_has_been_wished_for
                    || (some_card_has_been_wished_for && !user_can_play_wished_for_card)
                    || (some_card_has_been_wished_for
                        && user_can_play_wished_for_card
                        && combo_contains_wished_for_card))
        } else {
            // cards are not a valid combo
            false
        }
    }

    pub fn stage_is_trade(&self) -> bool {
        matches!(
            self.game_state.as_ref().unwrap().stage,
            PublicGameStage::Trade(_)
        )
    }

    pub fn stage_is_play(&self) -> bool {
        matches!(
            self.game_state.as_ref().unwrap().stage,
            PublicGameStage::Play(_)
        )
    }

    pub fn is_current_users_turn(&self) -> bool {
        if let Some(game_state) = &self.game_state {
            if let PublicGameStage::Play(play_state) = &game_state.stage {
                return play_state.turn_user_id == self.user_id;
            }
        }
        false
    }

    pub fn is_play_card_selected(&self, card: &Card) -> bool {
        self.selected_play_cards.contains(card)
    }

    pub fn can_select_play_card(&self) -> bool {
        self.stage_is_play()
    }

    pub fn hand_beats_combo_on_table(&self, next_combo: &ValidCardCombo) -> bool {
        let prev_combo = if let Some(game_state) = &self.game_state {
            if let PublicGameStage::Play(play_stage) = &game_state.stage {
                play_stage.table.last()
            } else {
                // game stage is not Play
                return false;
            }
        } else {
            // game state is not instantiated
            return false;
        };
        next_combo_beats_prev(&prev_combo, next_combo)
    }

    pub fn can_call_or_decline_grand_tichu(&self) -> bool {
        if let Some(game_state) = &self.game_state {
            if let PublicGameStage::GrandTichu(grand_tichu_stage) = &game_state.stage {
                grand_tichu_stage.grand_tichus.iter().any(|call_status| {
                    *call_status.user_id == self.user_id
                        && matches!(call_status.tichu_call_status, TichuCallStatus::Undecided)
                })
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn can_call_small_tichu(&self) -> bool {
        // hasn't yet played a card
        let has_played_first_card = if let Some(game_state) = &self.game_state {
            game_state.current_user.has_played_first_card
        } else {
            false
        };

        let game_stage_is_active = if let Some(game_state) = &self.game_state {
            game_state.stage.is_active()
        } else {
            false
        };

        let undecided_about_small_tichu = if let Some(game_state) = &self.game_state {
            let small_tichus = match &game_state.stage {
                PublicGameStage::GrandTichu(public_grand_tichu) => &public_grand_tichu.small_tichus,
                PublicGameStage::Trade(trade) => &trade.small_tichus,
                PublicGameStage::Play(play) => &play.small_tichus,
                // game is not active, therefore can't call small tichu
                _ => return false,
            };

            if let Some(call_status) = small_tichus.iter().find(|user_id_with_tichu_call_status| {
                *user_id_with_tichu_call_status.user_id == *self.user_id
            }) {
                matches!(call_status.tichu_call_status, TichuCallStatus::Undecided)
            } else {
                // user's call status not found, therefore undecided
                true
            }
        } else {
            // GameState is None, therefore user is Undecided
            true
        };

        !has_played_first_card && undecided_about_small_tichu && game_stage_is_active
    }

    pub fn is_team_stage(&self) -> bool {
        match &self.game_state {
            None => false,
            Some(game_state) => matches!(&game_state.stage, PublicGameStage::Teams(_)),
        }
    }

    pub fn is_on_team_a(&self) -> bool {
        match &self.game_state {
            None => false,
            Some(game_state) => match &game_state.stage {
                PublicGameStage::Teams(teams) => teams[0].user_ids.contains(&self.user_id),
                _ => false,
            },
        }
    }

    pub fn is_on_team_b(&self) -> bool {
        match &self.game_state {
            None => false,
            Some(game_state) => match &game_state.stage {
                PublicGameStage::Teams(teams) => teams[1].user_ids.contains(&self.user_id),
                _ => false,
            },
        }
    }

    pub fn is_current_user_owner(&self) -> bool {
        match &self.game_state {
            None => false,
            Some(game_state) => game_state.owner_id == self.user_id,
        }
    }

    pub fn can_start_game(&self) -> bool {
        let current_user_is_owner = self.is_current_user_owner();
        let mut teams_are_ready = false;
        if let Some(game_state) = &self.game_state {
            if let PublicGameStage::Teams(teams_state) = &game_state.stage {
                if teams_state[0].user_ids.len() == 2 && teams_state[1].user_ids.len() == 2 {
                    teams_are_ready = true;
                }
            }
        }
        current_user_is_owner && teams_are_ready
    }

    pub fn can_submit_trade(&self) -> bool {
        // state is trade
        if let Some(game_state) = &self.game_state {
            if let PublicGameStage::Trade(_) = &game_state.stage {
                // there is a card assigned to every user
                self.trade_to_opponent1.is_some()
                    && self.trade_to_teammate.is_some()
                    && self.trade_to_opponent2.is_some()
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn get_can_pass(&self) -> bool {
        if let Some(game_state) = &self.game_state {
            // game stage is Play
            if let PublicGameStage::Play(play_state) = &game_state.stage {
                // it is the users turn
                if play_state.turn_user_id == self.user_id {
                    // user doesn't have to choose an opponent OR does have to choose an opponent and has done so
                    return !self.get_user_must_select_user_id_to_give_dragon_to()
                        || self.get_user_has_selected_user_id_to_give_dragon_to();
                }
            }
        }

        false
    }

    pub fn get_user_must_select_user_id_to_give_dragon_to(&self) -> bool {
        let opponent_ids = self.get_opponent_ids();
        if let Some(game_state) = &self.game_state {
            // game stage is Play
            if let PublicGameStage::Play(play_state) = &game_state.stage {
                // this is the last user to pass

                if play_state.passes.iter().filter(|pass| pass.passed).count() == 3 {
                    // last trick has a dragon in it
                    if play_state
                        .table
                        .last()
                        .expect("There should be cards on the table")
                        .cards()
                        .contains(&DRAGON)
                    {
                        let (opponent_id_0, opponent_id_1) =
                            opponent_ids.expect("Opponents should be found in state");
                        // user has chosen SOME user to give the dragon to
                        return if let Some(user_id_to_give_dragon_to) =
                            &self.user_id_to_give_dragon_to
                        {
                            // user has chosen an opponent to give the dragon to
                            !(user_id_to_give_dragon_to == &opponent_id_0
                                || user_id_to_give_dragon_to == &opponent_id_1)
                        } else {
                            // user has NOT chosen an opponent to give the dragon to
                            true
                        };
                    }
                }
            }
        }

        false
    }

    pub fn get_user_has_selected_user_id_to_give_dragon_to(&self) -> bool {
        let opponent_ids = self.get_opponent_ids();
        if let Some(game_state) = &self.game_state {
            // game stage is Play
            if let PublicGameStage::Play(play_state) = &game_state.stage {
                // this is the last user to pass

                if play_state.passes.iter().filter(|pass| pass.passed).count() == 3 {
                    // last trick has a dragon in it
                    if play_state
                        .table
                        .iter()
                        .any(|combo| combo.cards().contains(&DRAGON))
                    {
                        let (opponent_id_0, opponent_id_1) =
                            opponent_ids.expect("Opponents should be found in state");
                        // user has chosen an opponent to give the dragon to
                        return if let Some(user_id_to_give_dragon_to) =
                            &self.user_id_to_give_dragon_to
                        {
                            user_id_to_give_dragon_to == &opponent_id_0
                                || user_id_to_give_dragon_to == &opponent_id_1
                        } else {
                            false
                        };
                    }
                }
            }
        }

        false
    }

    fn get_opponent_ids(&self) -> Option<(String, String)> {
        if let Some(game_state) = &self.game_state {
            if let Some(TeamCategories { opposing_team, .. }) =
                game_state.get_immutable_team_categories()
            {
                Some((
                    opposing_team.user_ids[0].clone(),
                    opposing_team.user_ids[1].clone(),
                ))
            } else {
                None
            }
        } else {
            None
        }
    }
}

pub fn use_setup_app_reducer() -> UseReducerHandle<AppState> {
    use_reducer_eq(|| {
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
    })
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
