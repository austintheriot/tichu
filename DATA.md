# Data Contract Blueprint

## Todos / Ideas:
- CSS for Yew:
  - CSSinRust: https://crates.io/crates/css-in-rust
- Yew resources: 
  - Awesome Yew: https://github.com/jetli/awesome-yew

## Architecture


Two types of users:
- Owner - Whoever creates the room
- Participant - Whoever joins the room after the owner


Sockets {
  [IP Address?]: Socket,
},

enum TichuStatus {
  Called,
  Achieved,
  Failed,
}

Game {
  id: string,
  stage: Join,
  small_tichus: TichuStatus[],
  grand_tichus: TichuStatus[],
  participants: User[],
  teams: {
    [id: TeamId]: Team
  },
  owner_id: string,
  deck: Deck,
  active_player: string,
  card_wished_for: Card,
}

Table {
  deck: Card[],
  discard: Card[], 
  in_play: Card[],
}

Deck {
  cards: Card[],
}

Team {
  id: string,
  users: string[],
  score: number,
}

enum CardValues {
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    _10,
    Jack,
    Queen,
    King,
    Ace,
}

enum CardSuits {
    Sword(CardValues),
    Jade(CardValues),
    Pagoda(CardValues),
    Star(CardValues),
    Dragon,
    Phoenix,
    MahJong,
    Dog,
}

Trade { 
  from: userId,
  to: userId,
  card: Card,
}


enum Stage {
  Pregame,
  Teams,
  Trade,
  Game,
  Postgame,
}


Trick = Card[],

User {
  id: UID,
  role: UserType,
  display_name: string,
  hand: Card[],
  tricks: Card[][],
  is_owner: boolean,
}

enum UserRole {
  Owner,
  Participant,
}


Client actions: 
 -   Create game
     -   user_id
     -   display_name
 -   Join game
     -   user_id
     -   display_name
     -   game_id
 -   Choose team
     -   user_id
     -   team_id
 -   Rename team
     -   team_name
 -   Start game
 -   Submit card trade
 -   Play cards 
     -   Typically only played during a turn, but if a bomb is selected, then can be played any time.
     -   Optional: 
         -   wished_for (when Mah Jong is played)
- Give Dragon

Server actions:
- Game create
- Stage changed
- Team renamed
- User moved teams
- Small Tichu called
- Grand Tichu called
- Start / Restart game 
  - Deal First 9 cards
- Deal Final 9 Cards
  - Player with Mah Jong leads
- Trade cards (after all submitted) / Begin play
- Cards played (updates cards in play, changes turn to the appropriate person)
- Dragon was won
- Player received Dragon
- End game