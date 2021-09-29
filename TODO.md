# Todos / Ideas:

CSS SOLUTIONS:
- CSS for Yew:
  - CSSinRust: https://crates.io/crates/css-in-rust
- Yew resources: 
  - Awesome Yew: https://github.com/jetli/awesome-yew

UNIQUE FEATURES:
- Add new special cards?

IDEAS:
- Ngrok for local hosting


--------------------------------------------------------------------------------------------

MUST HAVES:
- Deck should not be sent in game state to other participants. This will require distinguishing between public and private states for various Game Stages.
- Minimize .expect() errors (see example in call_grand_tichu.rs for better match expressions)

NICE TO HAVES:
- DRY up code around moving between Team A/B & renaming Team A/B
- Implement info routes: get current state of server: Connections, GameState, GameCodes
- Send WS messages concurrently? Especially when sending to group?
- Spawn ping/pong on ws upgrade for more randomized ping/pong distribution

OPTIONAL:
- Mark is_alive as true anytime any message is sent from client??
- Rethink use of Mutexes and RwLocks -- are interactions with state actually read-heavy or not?
- Use parking_lot Mutexes and RwLocks?
