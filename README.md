# Pop.tg URL shortener

![CF Badge](https://img.shields.io/static/v1?label=Powered%20By&message=Workers&color=E77A1E&style=for-the-badge)
![Uptime Badge](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/George-Miao/Upptime/master/api/pop-tg-api/uptime.json&style=for-the-badge)
![Response Time Badge](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/George-Miao/Upptime/master/api/pop-tg-api/response-time.json&style=for-the-badge)

## About <a name = "about"></a>

A simple, easy-to-use and free URL shortener built with cf-worker.

## Project Structure

`backend`: Legacy V1 API, not using or deployed anymore

`frontend`: www.pop.tg frontend, built using svelte

`v2api`: V2 API, naive RPC-like syntax for various method call, deployed on Cloudflare Workers

`cli`: CLI written in Rust to interact with API

## TODO

### CLI features

- [x] Create new
- [x] List local
- [ ] Query (Get one record by key)
- [ ] List remote (Get multiple records w/ or w/o cursor)
- [ ] Load and dump
- [ ] Manage
  - [ ] Update
  - [ ] Delete

### Namespace

_Maybe_ something like https://pop.tg/MyName/key, but that would require a user registration system, which I'm too lazy to implement
