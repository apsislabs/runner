FROM rust:1.43.1-slim

ENV APP_HOME /app
WORKDIR $APP_HOME

RUN rustup target install x86_64-unknown-linux-musl
# RUN rustup target install arm-unknown-linux-gnueabihf

CMD ["cargo", "build", "--release", "--target", "x86_64-unknown-linux-musl"]