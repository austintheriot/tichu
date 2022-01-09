# Tichu

## About

Tichu is an adaptation of the Chinese game *Zheng Fen* by Urs Hostettler and was released in 1991. It is a strategically complex game and is quite addicting. Watch out if you ever learn it! It might leave you reluctant to play other card games...

Copyright of this game belongs to the original publisher of Tichu, and I do not claim any copyright on the underlying material/mechanics of the game. This project was made for educational purposes only—namely that of learning Rust. 

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