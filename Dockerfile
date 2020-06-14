FROM ubuntu:18.04

COPY target/debug/tsurezure /app/tsurezure

ENTRYPOINT ["/app/tsurezure"]
WORKDIR "/app"