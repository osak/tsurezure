FROM ubuntu:18.04

COPY Rocket.toml /app/Rocket.toml
COPY target/debug/tsurezure /app/tsurezure

ENTRYPOINT ["/app/tsurezure"]
WORKDIR "/app"