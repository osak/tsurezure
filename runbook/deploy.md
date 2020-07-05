# Deployment

## Set password in env var
Admin APIs are protected by Basic auth. Set `ADMIN_USER` and `ADMIN_PASS` env var.

# Set secret key
Auth cookie is encrypted using `COOKIE_KEY`. It must be at least 32 characters long.

## Server deploy
`git push heroku master`