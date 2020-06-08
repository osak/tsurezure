FROM ubuntu:18.04

COPY target/debug/tsurezure /usr/local/bin

ENTRYPOINT ["/usr/local/bin/tsurezure"]