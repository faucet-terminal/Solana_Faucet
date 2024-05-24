FROM ubuntu:22.04

WORKDIR /app

COPY ./target/release/solana_faucet .

ENTRYPOINT ["/app/solana_faucet"]