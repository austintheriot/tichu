# Tichu

## About

Tichu is an adaptation of the Chinese game *Zheng Fen* by Urs Hostettler and was released in 1991. It is a strategically complex game and is quite addicting. 

This app is a fullstack multiplayer web app version of the game, allowing players to play online from via their browser. The backend uses the Warp framework for handling websocket connections and synchronizing game state, while the front end is written using the Yew framework and is compiled to WebAssembly for running in-browser. The two communicate via websocket messages in binary (using Serde for quick binary serialization and deserialization on either end). All shared game logic resides in the `/common` directory for easy code and type sharing.

Copyright of this game belongs to the original publisher of Tichu, and I do not claim copyright on any of the underlying material/mechanics of the game. This project was made for educational purposes only—namely that of learning Rust. 

## Scripts
- Prerequisite for running scripts in watch mode: 
```
 cargo install cargo-watch
```

- To run /server in watch mode: 
```
cd server
cargo watch -x run
```

- To build /common files in watch mod:
```
cd common
cargo watch -x build
```

- To run /client in watch mode:
```
cd client
npm start
```

- To build docker container: 
```
docker build -t tichu .
```

- To run docker container: 
```
docker run -it --name tichu --rm -p 8080:8080 tichu
```