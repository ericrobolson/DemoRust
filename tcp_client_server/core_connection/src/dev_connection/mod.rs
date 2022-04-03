mod client;
mod server;
mod streams;

pub use client::*;
pub use server::*;

use crate::PacketBuffer;

pub const SERVER_ADDR: &'static str = "127.0.0.1:8090";

#[derive(Debug, PartialEq, Eq)]
pub enum State {
    Send,
    Receive,
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
struct Secret {
    pub secret: String,
}

// General flow is this in order:
// 1 Client connects to server
// 2 Client sends over their secret
// 3 Server creates a new client with that secret
// 4 Server sends message to client
// 5 Client reads message from server
// 6 Client sends message to server
// 7 Server reads message from client
// Repeat 4-7 ad nauseam
// If there are no queued messages, send heartbeats of empty packets.
