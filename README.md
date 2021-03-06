# `openra-master-server-rs`

A reimplementation of the OpenRA Master Server written in Rust

### Usage

```
$ cargo run

🔧  Configured for development.
    => address: localhost
    => port: 8000
    => log: normal
    => workers: 4
🛰  Mounting '/':
    => GET /ping
🚀  Rocket has launched from http://localhost:8000...
```

Send a GET request to the `ping` endpoint:

```
localhost:8000/ping?port=8000&name=foo&state=1&players=4&bots=0&mods=ra@release-1234&maxplayers=6&spectators=0&protected=0&clients=10&map=mymap.oramap&new=1
```

```
GET /ping?port=8000&name=foo&state=1&players=4&bots=0&mods=ra@release-1234&maxplayers=6&spectators=0&protected=0&clients=10&map=mymap.oramap&new=1:
    => Matched: GET /ping
PingData { port: 8000, name: "foo", state: 1, players: 4, bots: 0, mods: "ra@release-1234", map: "mymap.oramap", maxplayers: 6, spectators: 0, protected: 0, clients: 10, n
ew: Some(1) }
V6([::1]:54693)
    => Outcome: Success
    => Response succeeded.
```