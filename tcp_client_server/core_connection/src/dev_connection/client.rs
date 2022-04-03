use std::net::TcpStream;

use core_time::Timer;

use super::{streams::*, PacketBuffer, Secret, State, SERVER_ADDR};

/// A client for the game.
/// Uses in order delivery for packets.
pub struct Client {
    secret: Secret,
    tcp: Option<TcpStream>,
    state: State,
    messages: Vec<PacketBuffer>,
    timer: Timer,
}
impl Client {
    /// Creates a new dev connection client.
    pub fn new<'a>(secret: &'a str, tick_rate: u32) -> Self {
        let mut client = Self {
            secret: Secret {
                secret: secret.to_string(),
            },
            timer: Timer::new(tick_rate),
            tcp: None,
            state: State::Send,
            messages: vec![],
        };

        client.reconnect();

        client
    }

    /// Polls the server for packets.
    pub fn poll(&mut self) -> Option<PacketBuffer> {
        if self.timer.tick().triggered {
            if let Some(conn) = &mut self.tcp {
                let result = poll(conn, &mut self.state, &mut self.messages);

                if result.reconnect {
                    self.reconnect();
                }

                result.msg
            } else {
                self.reconnect();
                None
            }
        } else {
            None
        }
    }

    /// Sends a packet to the server.
    pub fn send(&mut self, packet: PacketBuffer) {
        self.messages.push(packet);
    }

    /// Attempts to reconnect to the server.
    fn reconnect(&mut self) {
        match TcpStream::connect(SERVER_ADDR) {
            Ok(mut stream) => {
                if init_stream(&mut stream).success {
                    match PacketBuffer::from_str(&self.secret.secret) {
                        Some(buf) => {
                            if send_msg(&mut stream, buf).success {
                                self.tcp = Some(stream);
                                self.state = State::Receive;
                            }
                        }
                        None => todo!("Client secret too long!"),
                    }
                }
            }
            Err(error) => {
                println!("Error connecting: {:?}", error);
                self.state = State::Send;
            }
        }
    }
}
