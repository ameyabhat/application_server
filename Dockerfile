FROM rust:latest

RUN cd / &&\
	cargo new app 
WORKDIR /app

COPY Cargo.toml Cargo.lock sqlx-data.json ./
ENV SQLX_OFFLINE true
ADD .local.env ./.env

RUN cargo build
RUN rm -r src/

COPY ./src src
RUN touch src/main.rs && cargo build --release

EXPOSE 8000
