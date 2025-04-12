FROM rust:1.81.0 

WORKDIR /app

COPY . .

RUN cargo build --release