# Demo TCP server
This is a simple TCP server. It is not optimized, but shows how one can build a connection mechanism in Rust.

## Iteration Loop
This application is compromised of several modules:
- `client` a dummy client that talks to the `server`. For single player, it can also directly reference `core_game`
- `server` the server representation of the game. 
- `core_connection` is a library that represents a simple connection between a client and a server.
- `core_game` is a `no_std` library that represents the game.

The server can be restarted at any point, reload the state, then talk to the client. This enables a mechanism similar to hot reloading.

## Design Experiments
- Make things return structs as results. Make those objects with methods/other things to make this a more 'functional' type project.

# Misc tasks
- [x] Send packets at a set rate, e.g. 60hz
- [ ] Make heartbeats only send 4 bytes or something
