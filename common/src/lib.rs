use serde::{Deserialize, Serialize};

pub const NO_ID: &str = "no_id";

#[derive(Debug, Serialize, Deserialize)]
enum TichuCallStatus {
    Called,
    Achieved,
    Failed,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GameStage {
    Join,
    Pregame,
    GrandTichu,
    Trade,
    Game,
    Endgame,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    id: String,
    stage: GameStage,
    small_tichus: Vec<TichuCallStatus>,
    grand_tichus: Vec<TichuCallStatus>,
    participants: Vec<User>,
    teams: [Team; 2],
    owner_id: String,
    active_player: String,
    card_wished_for: Card,
    deck: Vec<Card>,
    discard: Vec<Card>,
    in_play: Vec<Card>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Team {
    id: String,
    users: Vec<String>,
    score: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CardValue {
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    _10,
    J,
    Q,
    K,
    A,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum Card {
    Sword(CardValue),
    Jade(CardValue),
    Pagoda(CardValue),
    Star(CardValue),
    Dragon,
    Phoenix,
    MahJong,
    Dog,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Trade {
    from: String,
    to: String,
    card: Card,
}

#[derive(Debug, Serialize, Deserialize)]
enum Stage {
    Pregame,
    Teams,
    Trade,
    Game,
    Postgame,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UserRole {
    Owner,
    Participant,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: String,
    role: UserRole,
    display_name: String,
    hand: Vec<Card>,
    tricks: Vec<Vec<Card>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateGameMessage {
    user_id: String,
    display_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JoinGameMessage {
    user_id: String,
    display_name: String,
    game_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChooseTeamMessage {
    team_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RenameTeamMessage {
    team_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitTradeMessage {
    trade_to: String,
    card: Card,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayCardsMessage {
    cards: Vec<Card>,
    wished_for: Option<Card>,
    give_dragon_to: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GiveDragonMessage {
    user_id: String,
}

/// Client to Server Websocket Messages
#[derive(Debug, Serialize, Deserialize)]
pub enum CTSMsg {
    CreateGame(CreateGameMessage),
    JoinGame(JoinGameMessage),
    ChooseTeam(ChooseTeamMessage),
    RenameTeam(RenameTeamMessage),
    StartGame,
    SubmitTrade(SubmitTradeMessage),
    PlayCards(PlayCardsMessage),
    GiveDragon(GiveDragonMessage),

    Ping,
    Pong,
    Test(String),
}

/// Server to Client Websocket Messages
#[derive(Debug, Serialize, Deserialize)]
pub enum STCMsg {
    State(Game),
    UserIdAssigned(String),
    GameCreated,
    GameStateChanged,
    TeamRenamed,
    SmallTichuCalled,
    GrandTichuCalled,
    StartGame,
    // deal first 9 cards
    DealFinalCards,
    // deal last 5 cards
    // Player with the Mah Jon leads
    CardsTraded,
    // after all submitted
    CardsPlayed,
    DragonWasWon,
    PlayerReceivedDragon,
    EndGame,

    Ping,
    Pong,
    Test(String),
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
