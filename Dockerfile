FROM rust:latest

RUN cd / &&\
	cargo new app 
WORKDIR /app

COPY . .
ENV SQLX_OFFLINE true

RUN cargo build
RUN rm -r src/

ENV APP_ENVIRONMENT docker

COPY ./src src
RUN touch src/main.rs && cargo build --release

EXPOSE 8000
ENTRYPOINT ["./target/release/generate-tech-app"]
