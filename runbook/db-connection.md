# How to connect to DB

## Local
1. Run database: `docker-compose up`
2. `psql -U tsurezure -h 127.0.0.1 -p 15432` (pass: tsurezure)

## Prod (Heroku)
1. `heroku psql`