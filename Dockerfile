FROM rust:latest

RUN cd / &&\
	cargo new app 
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
ADD .local.env ./.env

RUN cargo build
RUN rm -r src/

COPY ./src src
RUN touch src/main.rs && cargo build --release

EXPOSE 8000
ENTRYPOINT [ "./target/release/generate-tech-app" ]
