# Todos / Ideas:

CSS SOLUTIONS:
- CSS for Yew:
  - CSSinRust: https://crates.io/crates/css-in-rust
- Yew resources: 
  - Awesome Yew: https://github.com/jetli/awesome-yew
- By hand: 
  - Just append raw css string to style tag in the head of the document

UNIQUE FEATURES:
- Add new special cards?

IDEAS:
- Ngrok for local hosting


--------------------------------------------------------------------------------------------

TODO:
- "wished for card" should only be a VALUE - not a card
- account for "wished for card" in the client--i.e. if a user CAN play a card that has been wished for, they can't submit PlayCard until they've chosen a combination that includes that card
- create input for wished for card?
- Show user_id_to_give_dragon_to input if a user is going to play the dragon
- Create input for user_id_to_give_dragon_to
- Only show "wish for a card" to user who is about to play mah jong
- Implement "waiting_for_user_id_to_give_dragon_to" in game_state
- Game state implementation for Pass move
- move more front-end state checks into /common (in prep for bot player)
  
- use "into" pattern more: 
- fn last_name(mut self, last_name: impl Into<String>) -> Self {
        self.last_name = Some(last_name.into());
        self
    }


MUST HAVES:
- Add client-side check before being able to call Grand Tichu
- Minimize .expect() errors (see example in call_grand_tichu.rs for better match expressions)
- update Small tichu to be able to be called from states other than Grand Tichu once other stages are implemented
- Search TODOs in codebase
- - make sort by value the default for Cards

NICE TO HAVES:
- Configure rand crate to "js" feature for randomness
- Redirect normal logs to println (to a file output) & reserve eprintln for errors
- Return Result<PrivateGameState, String> for GameState results
- Validate in handle_ws_message OR in game_state methods, but NOT BOTH --- OR share methods for determining if that action can be taken
- Validate SubmitTrade in websocket message (Share validations with PrivateGameState)
    - Conditionally send websocket messages based on results
- Allow users to change their Grand Tichu call before the last person has called it
- Show other users who have joined the game once a user has created / joined a room
- Convert "Call Small Tichu" into a reusable Component
- Apply only Arc to HashMap, but apply Arc and Mutex to each individual game state, etc. that way users are not locked from reading from the hashmaps
- DRY up code around moving between Team A/B & renaming Team A/B
- Implement info routes: get current state of server: Connections, GameState, GameCodes
- Send WS messages concurrently? Especially when sending to group?
- Spawn ping/pong on ws upgrade for more randomized ping/pong distribution

OPTIONAL:
- Mark is_alive as true anytime any message is sent from client??
- Rethink use of Mutexes and RwLocks -- are interactions with state actually read-heavy or not?
- Use parking_lot Mutexes and RwLocks?


BUGS:
- Server occasionally gets deadlocked when many users leave at once