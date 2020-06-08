#!/bin/bash

sed -i "s!{{DATABASE_URL}}!${DATABASE_URL}!" Rocket.toml
ROCKET_PORT=$PORT ./target/release/tsurezure