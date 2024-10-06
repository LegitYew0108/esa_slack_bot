FROM rust:1.80.1

WORKDIR /usr/src/app
COPY . .

RUN cargo install

CMD ["esa_slack_bot"]
