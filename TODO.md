# Todos / Ideas:
- CSS for Yew:
  - CSSinRust: https://crates.io/crates/css-in-rust
- Yew resources: 
  - Awesome Yew: https://github.com/jetli/awesome-yew

- input validation for display_name (client & server side) -- no empty strings, minimum length, etc.
- Accept UN-capitalized game codes (do good input validation: trim, capitalize, etc.)
- Call "Endgame" stage "Scoreboard" or something similar instead
- Spawn ping/pong on ws upgrade for more randomized ping/pong distribution
- Mark is_alive as true anytime any message is sent from client??
- Make ping/pong send interval shorter for production 
- Add display name inputs
- Ensure that users who disconnect and reconnect get added back into the Game properly and receive a Game state update
- Send state updates to ALL participants
- Don't send users' cards in the Game state to other users


Unique features:
- Add new special cards?

Ideas:
- Ngrok for local hosting