# rusty bote api

barebones api service for [rusty bote](https://github.com/kylegrover/rusty-bote) used to show fun stats on the website.

you don't need this to run your own rusty bote, but you can!

## what is this?
this is a tiny REST API (written in rust, using axum/sqlx) that exposes some stats about rusty bote's usage (guilds, polls, votes, etc). it's mainly for the web dashboard, but you can poke at it if you want.

## running locally
1. clone this repo
2. copy `.env` and set your `DATABASE_URL` (sqlite or postgres both work)
3. `cargo run`

it'll start on localhost:3000 by default. go to `localhost:3000/stats` for the json.

## endpoints
- `GET /stats` — returns stats like guild count, poll count, votes, and breakdowns by poll type

## notes
- you don't need this to run the bot itself, it's just for stats
- if you want to use sqlite, just set `DATABASE_URL=sqlite:rusty_bote_dev.db` in your `.env`
- prod uses postgres, but local sqlite is fine for testing