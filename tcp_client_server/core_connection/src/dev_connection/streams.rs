use std::{
    io::{Read, Write},
    net::TcpStream,
};

use crate::PacketBuffer;

use super::State;

pub fn send_msg(stream: &mut TcpStream, buffer: PacketBuffer) -> SendMsg {
    let mut result = SendMsg { success: false };

    match stream.write(&buffer.serialize()) {
        Ok(_) => result.success = true,
        Err(_) => {}
    }

    result
}
pub struct SendMsg {
    pub success: bool,
}

/// Attempts to read a message from the stream.
pub fn recv_msg(stream: &mut TcpStream) -> RecvMsg {
    let mut buf = PacketBuffer::empty_buffer();
    match stream.read(&mut buf) {
        Ok(_) => match PacketBuffer::deserialize(&buf) {
            Some(buf) => RecvMsg::Success(buf),
            None => RecvMsg::Disconnected,
        },
        Err(e) => {
            println!("{:?}", e);
            RecvMsg::Disconnected
        }
    }
}

pub enum RecvMsg {
    Disconnected,
    Success(PacketBuffer),
}

pub fn init_stream(stream: &mut TcpStream) -> InitStream {
    let success = stream.set_nodelay(true).is_ok();

    InitStream { success }
}
pub struct InitStream {
    pub success: bool,
}

#[derive(Debug)]
pub struct Poll {
    pub msg: Option<PacketBuffer>,
    pub reconnect: bool,
}

/// Polls the given stream.
/// Will send a message if the state is in send mode.
/// If no messages in message_queue, it will send a heartbeat packet.
pub fn poll(
    conn: &mut TcpStream,
    state: &mut State,
    message_queue: &mut Vec<PacketBuffer>,
) -> Poll {
    let mut result = Poll {
        msg: None,
        reconnect: false,
    };

    match state {
        State::Send => {
            let packet = if message_queue.is_empty() {
                // Send a empty packet as a heartbeat packet
                PacketBuffer::empty()
            } else {
                message_queue.remove(0)
            };

            if send_msg(conn, packet).success {
                *state = State::Receive;
            } else {
                result.reconnect = true;
            }
        }
        State::Receive => match recv_msg(conn) {
            RecvMsg::Disconnected => {
                result.reconnect = true;
            }
            RecvMsg::Success(packet) => {
                *state = State::Send;

                // Discard empty packets as they are heartbeats.
                if packet.is_empty() == false {
                    result.msg = Some(packet)
                }
            }
        },
    }

    result
}
