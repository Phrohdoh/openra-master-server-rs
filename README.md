# `openra-master-server-rs`

A reimplementation of the OpenRA Master Server written in Rust

### Usage

```
$ cargo run

# now make a request to http://localhost:8000/ping/?port=8000&name=foo&state=1&players=4&bots=0&mods=ra@release-1234&maxplayers=6&spectators=0&protected=0&clients=10&map=mymap.oramap&new=1
```