# Tichu

## About

Tichu is a fun, strategically complex card game based on the traditional Chinese game *Zheng Fen*.

This app is a fullstack web version of the game, allowing users to play online via their browser. The backend uses the Warp framework for handling websocket connections and synchronizing game state, while the front end is written using the Yew framework and is compiled to WebAssembly for running in-browser. The two communicate via binary websocket messages (using Serde for quick serialization and deserialization on either end). All shared game logic resides in the `/common` directory for easy code and type sharing.

Although there are numerous editions of Tichu available for sale, copyright of this game appears to belong to the original publisher, Fata Morgana, and I do not claim copyright on any of the underlying material/mechanics of the game. This project was made for nonprofit, educational purposes only—primarily that of learning the Rust programming language.

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

- To run /server in watch mode (watching entire project): 
```
cd server
cargo watch -w ../ -x run
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

- To run /client in watch mode (watching entire project):
```
cd client
cargo watch -w ../ -- npm run start
```

- To build docker container: 
```
docker build -t tichu .
```

- To run docker container: 
```
docker run -it --name tichu --rm -p 8080:8080 tichu
```