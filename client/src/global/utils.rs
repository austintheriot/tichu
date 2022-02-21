use common::{PublicGameStage, PublicGameState, UserIdWithTichuCallStatus};

pub fn get_small_tichus<'a>(
    public_game_state: &'a PublicGameState,
) -> Option<&'a [UserIdWithTichuCallStatus; 4]> {
    match &public_game_state.stage {
        PublicGameStage::GrandTichu(grand_tichu_state) => Some(&grand_tichu_state.small_tichus),
        PublicGameStage::Trade(trade_state) => Some(&trade_state.small_tichus),
        PublicGameStage::Play(play_state) => Some(&play_state.small_tichus),
        _ => None,
    }
}

pub fn get_users_tichu_call_status<'a>(
    tichus: &'a [UserIdWithTichuCallStatus; 4],
    user_id: &str,
) -> Option<&'a UserIdWithTichuCallStatus> {
    tichus
        .iter()
        .find(|user_id_with_tichu_call_status| user_id_with_tichu_call_status.user_id == user_id)
}
