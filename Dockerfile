# base image
FROM rustlang/rust:nightly AS builder

# working directory inside the Container
WORKDIR /usr/src/tichu

# copy all files into the docker image
COPY . .

# install and build dependencies
RUN cd common && cargo build --release
RUN cd server && cargo build --release

# define the port number the container should expose
ENV PORT=8080
EXPOSE 8080

# startup the server
CMD cd server && target/release/server