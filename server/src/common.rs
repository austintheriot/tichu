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

#[derive(Debug)]
enum TichuCallStatus {
    Called,
    Achieved,
    Failed,
}

#[derive(Debug)]
pub enum GameStage {
    Join,
    Pregame,
    GrandTichu,
    Trade,
    Game,
    Endgame,
}

#[derive(Debug)]
pub struct Game {
    id: Uuid,
    stage: GameStage,
    small_tichus: Vec<TichuCallStatus>,
    grand_tichus: Vec<TichuCallStatus>,
    participants: Vec<User>,
    teams: [Team; 2],
    owner_id: Uuid,
    active_player: Uuid,
    card_wished_for: Card,
    deck: Vec<Card>,
    discard: Vec<Card>,
    in_play: Vec<Card>,
}

#[derive(Debug)]
pub struct Team {
    id: Uuid,
    users: Vec<Uuid>,
    score: i32,
}

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Trade {
    from: Uuid,
    to: Uuid,
    card: Card,
}

#[derive(Debug)]
enum Stage {
    Pregame,
    Teams,
    Trade,
    Game,
    Postgame,
}

#[derive(Debug)]
pub enum UserRole {
    Owner,
    Participant,
}

#[derive(Debug)]
pub struct User {
    id: Uuid,
    role: UserRole,
    display_name: String,
    hand: Vec<Card>,
    tricks: Vec<Vec<Card>>,
}

#[derive(Debug)]
pub struct CreateGameMessage {
    user_id: Uuid,
    display_name: String,
}

#[derive(Debug)]
pub struct JoinGameMessage {
    user_id: Uuid,
    display_name: String,
    game_id: String,
}

#[derive(Debug)]
pub struct ChooseTeamMessage {
    team_id: Uuid,
}

#[derive(Debug)]
pub struct RenameTeamMessage {
    team_name: String,
}

#[derive(Debug)]
pub struct SubmitTradeMessage {
    trade_to: Uuid,
    card: Card,
}

#[derive(Debug)]
pub struct PlayCardsMessage {
    cards: Vec<Card>,
    wished_for: Option<Card>,
    give_dragon_to: Option<Uuid>,
}

#[derive(Debug)]
pub struct GiveDragonMessage {
    user_id: Uuid,
}

/// Client Websocket Messages to the Server
pub enum ClientToServerMessageData {
    CreateGame(CreateGameMessage),
    JoinGame(JoinGameMessage),
    ChooseTeam(ChooseTeamMessage),
    RenameTeam(RenameTeamMessage),
    StartGame,
    SubmitTrade(SubmitTradeMessage),
    PlayCards(PlayCardsMessage),
    GiveDragon(GiveDragonMessage),
}

pub enum ServerToClientMessageData {
    State(Game),
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
}
